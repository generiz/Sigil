use blake3::Hash;
use rand::{rngs::OsRng, seq::SliceRandom, RngCore};
use reed_solomon_erasure::galois_8::ReedSolomon;
use std::{collections::{HashMap, HashSet}, error::Error, fmt};
use zeroize::Zeroize;

const MIN_DATA_SHARDS: usize = 2;
const MIN_PARITY_SHARDS: usize = 1;
const MAX_TOTAL_SHARDS: usize = 255;
const CAPABILITY_LEN: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FragmentCapability([u8; CAPABILITY_LEN]);

impl FragmentCapability {
    fn random() -> Self {
        let mut bytes = [0u8; CAPABILITY_LEN];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; CAPABILITY_LEN] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentPolicy {
    data_shards: usize,
    parity_shards: usize,
}

impl FragmentPolicy {
    pub fn new(data_shards: usize, parity_shards: usize) -> Result<Self, FragmentError> {
        if data_shards < MIN_DATA_SHARDS
            || parity_shards < MIN_PARITY_SHARDS
            || data_shards + parity_shards > MAX_TOTAL_SHARDS
        {
            return Err(FragmentError::InvalidPolicy);
        }

        Ok(Self {
            data_shards,
            parity_shards,
        })
    }

    pub fn data_shards(self) -> usize {
        self.data_shards
    }

    pub fn parity_shards(self) -> usize {
        self.parity_shards
    }

    pub fn total_shards(self) -> usize {
        self.data_shards + self.parity_shards
    }
}

impl Default for FragmentPolicy {
    fn default() -> Self {
        Self {
            data_shards: 12,
            parity_shards: 8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpaqueFragment {
    capability: FragmentCapability,
    payload: Vec<u8>,
}

impl OpaqueFragment {
    pub fn capability(&self) -> &FragmentCapability {
        &self.capability
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestEntry {
    capability: FragmentCapability,
    shard_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FragmentManifest {
    original_len: usize,
    shard_len: usize,
    policy: FragmentPolicy,
    ciphertext_digest: [u8; 32],
    entries: Vec<ManifestEntry>,
}

impl FragmentManifest {
    pub fn required_fragments(&self) -> usize {
        self.policy.data_shards()
    }

    pub fn total_fragments(&self) -> usize {
        self.policy.total_shards()
    }

    pub fn shard_len(&self) -> usize {
        self.shard_len
    }

    pub fn reconstruct(
        &self,
        received: &[OpaqueFragment],
    ) -> Result<Vec<u8>, FragmentError> {
        if received.len() < self.required_fragments() {
            return Err(FragmentError::InsufficientFragments);
        }

        let by_capability: HashMap<[u8; CAPABILITY_LEN], usize> = self
            .entries
            .iter()
            .map(|entry| (*entry.capability.as_bytes(), entry.shard_index))
            .collect();

        let mut seen_indices = HashSet::with_capacity(received.len());
        let mut shards: Vec<Option<Vec<u8>>> =
            (0..self.total_fragments()).map(|_| None).collect();

        for fragment in received {
            if fragment.payload.len() != self.shard_len {
                return Err(FragmentError::InvalidFragmentLength);
            }

            let index = by_capability
                .get(fragment.capability.as_bytes())
                .copied()
                .ok_or(FragmentError::UnknownFragment)?;

            if !seen_indices.insert(index) {
                return Err(FragmentError::DuplicateFragment);
            }

            shards[index] = Some(fragment.payload.clone());
        }

        if seen_indices.len() < self.required_fragments() {
            return Err(FragmentError::InsufficientFragments);
        }

        let codec = ReedSolomon::new(
            self.policy.data_shards(),
            self.policy.parity_shards(),
        )
        .map_err(|_| FragmentError::CodingFailure)?;

        codec
            .reconstruct(&mut shards)
            .map_err(|_| FragmentError::CodingFailure)?;

        let mut reconstructed = Vec::with_capacity(self.policy.data_shards() * self.shard_len);
        for shard in shards.iter().take(self.policy.data_shards()) {
            let bytes = shard.as_ref().ok_or(FragmentError::CodingFailure)?;
            reconstructed.extend_from_slice(bytes);
        }
        reconstructed.truncate(self.original_len);

        if Hash::from(self.ciphertext_digest) != blake3::hash(&reconstructed) {
            reconstructed.zeroize();
            return Err(FragmentError::DigestMismatch);
        }

        Ok(reconstructed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FragmentBundle {
    manifest: FragmentManifest,
    fragments: Vec<OpaqueFragment>,
}

impl FragmentBundle {
    pub fn split(ciphertext: &[u8], policy: FragmentPolicy) -> Result<Self, FragmentError> {
        if ciphertext.is_empty() {
            return Err(FragmentError::EmptyCiphertext);
        }

        FragmentPolicy::new(policy.data_shards(), policy.parity_shards())?;

        let shard_len = ciphertext.len().div_ceil(policy.data_shards());
        let padded_len = shard_len * policy.data_shards();
        let mut padded = vec![0u8; padded_len];
        padded[..ciphertext.len()].copy_from_slice(ciphertext);
        if padded_len > ciphertext.len() {
            OsRng.fill_bytes(&mut padded[ciphertext.len()..]);
        }

        let mut shards: Vec<Vec<u8>> = padded
            .chunks_exact(shard_len)
            .map(|chunk| chunk.to_vec())
            .collect();
        padded.zeroize();

        shards.extend(
            (0..policy.parity_shards()).map(|_| vec![0u8; shard_len]),
        );

        let codec = ReedSolomon::new(policy.data_shards(), policy.parity_shards())
            .map_err(|_| FragmentError::CodingFailure)?;
        codec
            .encode(&mut shards)
            .map_err(|_| FragmentError::CodingFailure)?;

        let mut capability_values = HashSet::with_capacity(policy.total_shards());
        let mut entries = Vec::with_capacity(policy.total_shards());
        let mut fragments = Vec::with_capacity(policy.total_shards());

        for (shard_index, payload) in shards.into_iter().enumerate() {
            let capability = loop {
                let candidate = FragmentCapability::random();
                if capability_values.insert(*candidate.as_bytes()) {
                    break candidate;
                }
            };

            entries.push(ManifestEntry {
                capability: capability.clone(),
                shard_index,
            });
            fragments.push(OpaqueFragment {
                capability,
                payload,
            });
        }

        fragments.shuffle(&mut OsRng);

        Ok(Self {
            manifest: FragmentManifest {
                original_len: ciphertext.len(),
                shard_len,
                policy,
                ciphertext_digest: *blake3::hash(ciphertext).as_bytes(),
                entries,
            },
            fragments,
        })
    }

    pub fn manifest(&self) -> &FragmentManifest {
        &self.manifest
    }

    pub fn fragments(&self) -> &[OpaqueFragment] {
        &self.fragments
    }

    pub fn into_parts(self) -> (FragmentManifest, Vec<OpaqueFragment>) {
        (self.manifest, self.fragments)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FragmentError {
    EmptyCiphertext,
    InvalidPolicy,
    InsufficientFragments,
    UnknownFragment,
    DuplicateFragment,
    InvalidFragmentLength,
    CodingFailure,
    DigestMismatch,
}

impl fmt::Display for FragmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCiphertext => write!(f, "ciphertext cannot be empty"),
            Self::InvalidPolicy => write!(f, "invalid fragment policy"),
            Self::InsufficientFragments => write!(f, "not enough fragments to reconstruct"),
            Self::UnknownFragment => write!(f, "fragment capability is not in the manifest"),
            Self::DuplicateFragment => write!(f, "duplicate fragment"),
            Self::InvalidFragmentLength => write!(f, "fragment length does not match manifest"),
            Self::CodingFailure => write!(f, "erasure coding failed"),
            Self::DigestMismatch => write!(f, "reconstructed ciphertext digest mismatch"),
        }
    }
}

impl Error for FragmentError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_recovers_from_eight_missing_pieces() {
        let ciphertext = (0..80_000).map(|i| (i % 251) as u8).collect::<Vec<_>>();
        let bundle = FragmentBundle::split(&ciphertext, FragmentPolicy::default()).unwrap();
        let available = bundle.fragments()[8..].to_vec();

        assert_eq!(available.len(), 12);
        assert_eq!(bundle.manifest().reconstruct(&available).unwrap(), ciphertext);
    }

    #[test]
    fn fragment_order_is_not_required_for_reconstruction() {
        let ciphertext = vec![0xA5; 19_003];
        let bundle = FragmentBundle::split(&ciphertext, FragmentPolicy::new(6, 4).unwrap()).unwrap();
        let mut available = bundle.fragments().to_vec();
        available.reverse();
        available.truncate(6);

        assert_eq!(bundle.manifest().reconstruct(&available).unwrap(), ciphertext);
    }

    #[test]
    fn network_fragment_does_not_expose_shard_index() {
        let bundle = FragmentBundle::split(&[7u8; 4096], FragmentPolicy::new(4, 2).unwrap()).unwrap();
        for fragment in bundle.fragments() {
            assert_eq!(fragment.capability().as_bytes().len(), CAPABILITY_LEN);
            assert_eq!(fragment.payload().len(), bundle.manifest().shard_len());
        }
    }

    #[test]
    fn corrupted_data_piece_is_detected_by_manifest_digest() {
        let ciphertext = (0..5000).map(|i| (i % 239) as u8).collect::<Vec<_>>();
        let mut bundle = FragmentBundle::split(&ciphertext, FragmentPolicy::new(4, 2).unwrap()).unwrap();

        let data_capability = bundle.manifest.entries[0].capability.clone();
        let fragment = bundle
            .fragments
            .iter_mut()
            .find(|fragment| fragment.capability == data_capability)
            .unwrap();
        fragment.payload[0] ^= 0x40;

        assert_eq!(
            bundle.manifest.reconstruct(&bundle.fragments),
            Err(FragmentError::DigestMismatch)
        );
    }

    #[test]
    fn fewer_than_threshold_fragments_are_rejected() {
        let bundle = FragmentBundle::split(&[3u8; 10_000], FragmentPolicy::new(5, 3).unwrap()).unwrap();
        assert_eq!(
            bundle.manifest().reconstruct(&bundle.fragments()[..4]),
            Err(FragmentError::InsufficientFragments)
        );
    }
}
