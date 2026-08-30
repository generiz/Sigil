use rand::{rngs::OsRng, RngCore};
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityPublicKey([u8; 32]);

impl IdentityPublicKey {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn fingerprint(&self) -> IdentityFingerprint {
        let mut hasher = blake3::Hasher::new_derive_key("sigil.identity.fingerprint.v1");
        hasher.update(&self.0);
        IdentityFingerprint(*hasher.finalize().as_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityFingerprint([u8; 32]);

impl IdentityFingerprint {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_grouped_hex(&self) -> String {
        let mut output = String::with_capacity(79);
        for (index, pair) in self.0.chunks_exact(2).enumerate() {
            if index > 0 {
                output.push(' ');
            }
            let _ = write!(output, "{:02X}{:02X}", pair[0], pair[1]);
        }
        output
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContactAlias([u8; 8]);

impl ContactAlias {
    pub fn random() -> Self {
        let mut rng = OsRng;
        loop {
            let mut bytes = [0u8; 8];
            rng.fill_bytes(&mut bytes);
            if bytes != [0u8; 8] {
                return Self(bytes);
            }
        }
    }

    pub fn display(&self) -> String {
        format!(
            "{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7]
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustState {
    Unverified,
    Verified,
    KeyChanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactRecord {
    alias: ContactAlias,
    identity: IdentityPublicKey,
    trust: TrustState,
}

impl ContactRecord {
    pub fn new(identity: IdentityPublicKey) -> Self {
        Self {
            alias: ContactAlias::random(),
            identity,
            trust: TrustState::Unverified,
        }
    }

    pub fn alias(&self) -> ContactAlias {
        self.alias
    }

    pub fn identity(&self) -> IdentityPublicKey {
        self.identity
    }

    pub fn fingerprint(&self) -> IdentityFingerprint {
        self.identity.fingerprint()
    }

    pub fn trust(&self) -> TrustState {
        self.trust
    }

    pub fn verify_observed_fingerprint(&mut self, observed: IdentityFingerprint) -> bool {
        if observed == self.fingerprint() {
            self.trust = TrustState::Verified;
            true
        } else {
            false
        }
    }

    pub fn rotate_alias(&mut self) {
        let previous = self.alias;
        loop {
            let next = ContactAlias::random();
            if next != previous {
                self.alias = next;
                return;
            }
        }
    }

    pub fn replace_identity(&mut self, identity: IdentityPublicKey) -> bool {
        if identity == self.identity {
            return false;
        }

        self.identity = identity;
        self.trust = TrustState::KeyChanged;
        self.rotate_alias();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(byte: u8) -> IdentityPublicKey {
        IdentityPublicKey::new([byte; 32])
    }

    #[test]
    fn alias_has_no_human_name_and_uses_fixed_random_code_shape() {
        let alias = ContactAlias::random().display();
        assert_eq!(alias.len(), 19);
        assert_eq!(alias.chars().filter(|c| *c == '-').count(), 3);
    }

    #[test]
    fn fingerprint_is_stable_for_the_same_public_identity() {
        assert_eq!(key(7).fingerprint(), key(7).fingerprint());
        assert_ne!(key(7).fingerprint(), key(8).fingerprint());
    }

    #[test]
    fn contact_becomes_verified_only_for_the_expected_fingerprint() {
        let mut contact = ContactRecord::new(key(1));
        assert!(!contact.verify_observed_fingerprint(key(2).fingerprint()));
        assert_eq!(contact.trust(), TrustState::Unverified);
        assert!(contact.verify_observed_fingerprint(key(1).fingerprint()));
        assert_eq!(contact.trust(), TrustState::Verified);
    }

    #[test]
    fn identity_change_invalidates_existing_trust() {
        let mut contact = ContactRecord::new(key(1));
        assert!(contact.verify_observed_fingerprint(key(1).fingerprint()));
        let previous_alias = contact.alias();

        assert!(contact.replace_identity(key(2)));
        assert_eq!(contact.trust(), TrustState::KeyChanged);
        assert_ne!(contact.alias(), previous_alias);
        assert!(!contact.verify_observed_fingerprint(key(1).fingerprint()));
        assert!(contact.verify_observed_fingerprint(key(2).fingerprint()));
        assert_eq!(contact.trust(), TrustState::Verified);
    }
}
