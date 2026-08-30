use blake3::keyed_hash;
use rand::{rngs::OsRng, RngCore};
use std::fmt;
use zeroize::Zeroize;

pub struct LocalVisualSecret([u8; 32]);

impl LocalVisualSecret {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for LocalVisualSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LocalVisualSecret([REDACTED])")
    }
}

impl Drop for LocalVisualSecret {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContactVisualMarker {
    pub palette_slot: u8,
    pub shape_slot: u8,
    pub pattern_slot: u8,
}

impl ContactVisualMarker {
    pub fn derive(identity_public_key: &[u8; 32], local_secret: &LocalVisualSecret) -> Self {
        let digest = keyed_hash(local_secret.as_bytes(), identity_public_key);
        let bytes = digest.as_bytes();

        Self {
            palette_slot: bytes[0] % 16,
            shape_slot: bytes[1] % 8,
            pattern_slot: bytes[2] % 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VisualEpochId([u8; 16]);

pub struct VisualRenderEpoch {
    id: VisualEpochId,
    render_token: [u8; 32],
}

impl fmt::Debug for VisualRenderEpoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VisualRenderEpoch")
            .field("id", &self.id)
            .field("render_token", &"[REDACTED]")
            .finish()
    }
}

impl VisualRenderEpoch {
    pub fn fresh() -> Self {
        let mut id = [0u8; 16];
        let mut render_token = [0u8; 32];
        OsRng.fill_bytes(&mut id);
        OsRng.fill_bytes(&mut render_token);

        Self {
            id: VisualEpochId(id),
            render_token,
        }
    }

    pub fn id(&self) -> VisualEpochId {
        self.id
    }

    pub(crate) fn render_token(&self) -> &[u8; 32] {
        &self.render_token
    }
}

impl Drop for VisualRenderEpoch {
    fn drop(&mut self) {
        self.render_token.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_is_stable_for_identity_and_local_secret() {
        let secret = LocalVisualSecret::random();
        let identity = [7u8; 32];
        assert_eq!(
            ContactVisualMarker::derive(&identity, &secret),
            ContactVisualMarker::derive(&identity, &secret)
        );
    }

    #[test]
    fn marker_changes_when_identity_changes() {
        let secret = LocalVisualSecret::random();
        let first = ContactVisualMarker::derive(&[1u8; 32], &secret);
        let second = ContactVisualMarker::derive(&[2u8; 32], &secret);
        assert_ne!(first, second);
    }

    #[test]
    fn render_epochs_rotate_internal_state() {
        let first = VisualRenderEpoch::fresh();
        let second = VisualRenderEpoch::fresh();
        assert_ne!(first.id(), second.id());
        assert_ne!(first.render_token(), second.render_token());
    }

    #[test]
    fn visual_secret_debug_output_is_redacted() {
        let secret = LocalVisualSecret::random();
        let epoch = VisualRenderEpoch::fresh();
        assert!(format!("{secret:?}").contains("REDACTED"));
        assert!(format!("{epoch:?}").contains("REDACTED"));
    }
}
