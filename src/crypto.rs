use blake3::keyed_hash;
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt,
};
use zeroize::{Zeroize, Zeroizing};

const WIRE_VERSION: u8 = 1;
const NONCE_LEN: usize = 24;
const AEAD_TAG_LEN: usize = 16;
const MAX_WIRE_BYTES: usize = 1024 * 1024;
const DEFAULT_REPLAY_WINDOW: usize = 4096;

pub struct MessageSecret([u8; 32]);

impl MessageSecret {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub(crate) fn derive_subkey(&self, label: &[u8]) -> [u8; 32] {
        *keyed_hash(&self.0, label).as_bytes()
    }
}

impl fmt::Debug for MessageSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MessageSecret([REDACTED])")
    }
}

impl Drop for MessageSecret {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

pub struct TransportSecret([u8; 32]);

impl TransportSecret {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    fn derive_subkey(&self, label: &[u8]) -> [u8; 32] {
        *keyed_hash(&self.0, label).as_bytes()
    }
}

impl fmt::Debug for TransportSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TransportSecret([REDACTED])")
    }
}

impl Drop for TransportSecret {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    AuthenticationFailed,
    InvalidKey,
    MalformedEnvelope,
    EnvelopeTooLarge,
    ReplayDetected,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthenticationFailed => write!(f, "authentication failed"),
            Self::InvalidKey => write!(f, "invalid encryption key"),
            Self::MalformedEnvelope => write!(f, "malformed layered envelope"),
            Self::EnvelopeTooLarge => write!(f, "layered envelope exceeds size limit"),
            Self::ReplayDetected => write!(f, "replayed layered envelope rejected"),
        }
    }
}

impl Error for CryptoError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayeredEnvelope {
    outer_nonce: [u8; NONCE_LEN],
    outer_ciphertext: Vec<u8>,
}

impl LayeredEnvelope {
    pub fn seal(
        symbol_stream: &[u8],
        message_secret: &MessageSecret,
        transport_secret: &TransportSecret,
        application_aad: &[u8],
        transport_aad: &[u8],
    ) -> Result<Self, CryptoError> {
        let inner_key = Zeroizing::new(message_secret.derive_subkey(b"sigil-inner-aead-v1"));
        let outer_key = Zeroizing::new(transport_secret.derive_subkey(b"sigil-outer-aead-v1"));

        let inner_cipher = XChaCha20Poly1305::new_from_slice(inner_key.as_ref())
            .map_err(|_| CryptoError::InvalidKey)?;
        let outer_cipher = XChaCha20Poly1305::new_from_slice(outer_key.as_ref())
            .map_err(|_| CryptoError::InvalidKey)?;

        let mut inner_nonce = [0u8; NONCE_LEN];
        let mut outer_nonce = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut inner_nonce);
        OsRng.fill_bytes(&mut outer_nonce);

        let inner_ciphertext = inner_cipher
            .encrypt(
                XNonce::from_slice(&inner_nonce),
                Payload {
                    msg: symbol_stream,
                    aad: application_aad,
                },
            )
            .map_err(|_| CryptoError::AuthenticationFailed)?;

        let mut inner_packet = Vec::with_capacity(1 + NONCE_LEN + inner_ciphertext.len());
        inner_packet.push(WIRE_VERSION);
        inner_packet.extend_from_slice(&inner_nonce);
        inner_packet.extend_from_slice(&inner_ciphertext);

        let outer_ciphertext = outer_cipher
            .encrypt(
                XNonce::from_slice(&outer_nonce),
                Payload {
                    msg: &inner_packet,
                    aad: transport_aad,
                },
            )
            .map_err(|_| CryptoError::AuthenticationFailed)?;

        inner_nonce.zeroize();
        inner_packet.zeroize();

        if 1 + NONCE_LEN + outer_ciphertext.len() > MAX_WIRE_BYTES {
            return Err(CryptoError::EnvelopeTooLarge);
        }

        Ok(Self {
            outer_nonce,
            outer_ciphertext,
        })
    }

    pub fn open(
        &self,
        message_secret: &MessageSecret,
        transport_secret: &TransportSecret,
        application_aad: &[u8],
        transport_aad: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        let inner_key = Zeroizing::new(message_secret.derive_subkey(b"sigil-inner-aead-v1"));
        let outer_key = Zeroizing::new(transport_secret.derive_subkey(b"sigil-outer-aead-v1"));

        let inner_cipher = XChaCha20Poly1305::new_from_slice(inner_key.as_ref())
            .map_err(|_| CryptoError::InvalidKey)?;
        let outer_cipher = XChaCha20Poly1305::new_from_slice(outer_key.as_ref())
            .map_err(|_| CryptoError::InvalidKey)?;

        let mut inner_packet = outer_cipher
            .decrypt(
                XNonce::from_slice(&self.outer_nonce),
                Payload {
                    msg: &self.outer_ciphertext,
                    aad: transport_aad,
                },
            )
            .map_err(|_| CryptoError::AuthenticationFailed)?;

        if inner_packet.len() < 1 + NONCE_LEN + AEAD_TAG_LEN || inner_packet[0] != WIRE_VERSION {
            inner_packet.zeroize();
            return Err(CryptoError::MalformedEnvelope);
        }

        let mut inner_nonce = [0u8; NONCE_LEN];
        inner_nonce.copy_from_slice(&inner_packet[1..1 + NONCE_LEN]);
        let inner_ciphertext = &inner_packet[1 + NONCE_LEN..];

        let result = inner_cipher
            .decrypt(
                XNonce::from_slice(&inner_nonce),
                Payload {
                    msg: inner_ciphertext,
                    aad: application_aad,
                },
            )
            .map_err(|_| CryptoError::AuthenticationFailed);

        inner_nonce.zeroize();
        inner_packet.zeroize();
        result
    }

    pub fn to_wire_bytes(&self) -> Vec<u8> {
        let mut wire = Vec::with_capacity(1 + NONCE_LEN + self.outer_ciphertext.len());
        wire.push(WIRE_VERSION);
        wire.extend_from_slice(&self.outer_nonce);
        wire.extend_from_slice(&self.outer_ciphertext);
        wire
    }

    pub fn from_wire_bytes(wire: &[u8]) -> Result<Self, CryptoError> {
        if wire.len() > MAX_WIRE_BYTES {
            return Err(CryptoError::EnvelopeTooLarge);
        }
        if wire.len() < 1 + NONCE_LEN + AEAD_TAG_LEN || wire[0] != WIRE_VERSION {
            return Err(CryptoError::MalformedEnvelope);
        }

        let mut outer_nonce = [0u8; NONCE_LEN];
        outer_nonce.copy_from_slice(&wire[1..1 + NONCE_LEN]);

        Ok(Self {
            outer_nonce,
            outer_ciphertext: wire[1 + NONCE_LEN..].to_vec(),
        })
    }
}

#[derive(Debug)]
pub struct ReplayGuard {
    capacity: usize,
    order: VecDeque<[u8; 32]>,
    seen: HashSet<[u8; 32]>,
}

impl Default for ReplayGuard {
    fn default() -> Self {
        Self::new(DEFAULT_REPLAY_WINDOW)
    }
}

impl ReplayGuard {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            order: VecDeque::new(),
            seen: HashSet::new(),
        }
    }

    pub fn open_once(
        &mut self,
        envelope: &LayeredEnvelope,
        message_secret: &MessageSecret,
        transport_secret: &TransportSecret,
        application_aad: &[u8],
        transport_aad: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        let digest = *blake3::hash(&envelope.to_wire_bytes()).as_bytes();
        if self.seen.contains(&digest) {
            return Err(CryptoError::ReplayDetected);
        }

        let opened = envelope.open(
            message_secret,
            transport_secret,
            application_aad,
            transport_aad,
        )?;

        self.seen.insert(digest);
        self.order.push_back(digest);
        while self.order.len() > self.capacity {
            if let Some(expired) = self.order.pop_front() {
                self.seen.remove(&expired);
            }
        }

        Ok(opened)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layered_envelope_roundtrips_binary_symbol_stream() {
        let message_secret = MessageSecret::random();
        let transport_secret = TransportSecret::random();
        let symbols = [0x91, 0x03, 0xA7, 0x44, 0x10, 0xEF];

        let envelope = LayeredEnvelope::seal(
            &symbols,
            &message_secret,
            &transport_secret,
            b"conversation-state",
            b"route-state",
        )
        .unwrap();

        let parsed = LayeredEnvelope::from_wire_bytes(&envelope.to_wire_bytes()).unwrap();
        let opened = parsed
            .open(
                &message_secret,
                &transport_secret,
                b"conversation-state",
                b"route-state",
            )
            .unwrap();

        assert_eq!(opened, symbols);
    }

    #[test]
    fn tampering_is_rejected_before_symbol_decode() {
        let message_secret = MessageSecret::random();
        let transport_secret = TransportSecret::random();
        let envelope = LayeredEnvelope::seal(
            &[1, 2, 3, 4],
            &message_secret,
            &transport_secret,
            b"app",
            b"transport",
        )
        .unwrap();

        let mut wire = envelope.to_wire_bytes();
        let last = wire.len() - 1;
        wire[last] ^= 0x80;
        let tampered = LayeredEnvelope::from_wire_bytes(&wire).unwrap();

        assert_eq!(
            tampered.open(&message_secret, &transport_secret, b"app", b"transport"),
            Err(CryptoError::AuthenticationFailed)
        );
    }

    #[test]
    fn independent_transport_secret_is_required() {
        let message_secret = MessageSecret::random();
        let correct_transport = TransportSecret::random();
        let wrong_transport = TransportSecret::random();
        let envelope = LayeredEnvelope::seal(
            &[9, 8, 7],
            &message_secret,
            &correct_transport,
            b"app",
            b"transport",
        )
        .unwrap();

        assert_eq!(
            envelope.open(&message_secret, &wrong_transport, b"app", b"transport"),
            Err(CryptoError::AuthenticationFailed)
        );
    }

    #[test]
    fn authenticated_context_is_bound_to_both_layers() {
        let message_secret = MessageSecret::random();
        let transport_secret = TransportSecret::random();
        let envelope = LayeredEnvelope::seal(
            &[5, 4, 3, 2, 1],
            &message_secret,
            &transport_secret,
            b"conversation-A",
            b"route-A",
        )
        .unwrap();

        assert_eq!(
            envelope.open(
                &message_secret,
                &transport_secret,
                b"conversation-B",
                b"route-A",
            ),
            Err(CryptoError::AuthenticationFailed)
        );
        assert_eq!(
            envelope.open(
                &message_secret,
                &transport_secret,
                b"conversation-A",
                b"route-B",
            ),
            Err(CryptoError::AuthenticationFailed)
        );
    }

    #[test]
    fn secret_debug_output_is_redacted() {
        let message = MessageSecret::random();
        let transport = TransportSecret::random();
        assert_eq!(format!("{message:?}"), "MessageSecret([REDACTED])");
        assert_eq!(format!("{transport:?}"), "TransportSecret([REDACTED])");
    }

    #[test]
    fn malformed_and_oversized_wire_objects_are_rejected() {
        assert_eq!(
            LayeredEnvelope::from_wire_bytes(&[WIRE_VERSION; NONCE_LEN]),
            Err(CryptoError::MalformedEnvelope)
        );
        let oversized = vec![0u8; MAX_WIRE_BYTES + 1];
        assert_eq!(
            LayeredEnvelope::from_wire_bytes(&oversized),
            Err(CryptoError::EnvelopeTooLarge)
        );
    }

    #[test]
    fn exact_authenticated_replay_is_rejected() {
        let message_secret = MessageSecret::random();
        let transport_secret = TransportSecret::random();
        let envelope = LayeredEnvelope::seal(
            &[4, 2, 4, 2],
            &message_secret,
            &transport_secret,
            b"app",
            b"transport",
        )
        .unwrap();
        let mut guard = ReplayGuard::default();

        assert_eq!(
            guard
                .open_once(
                    &envelope,
                    &message_secret,
                    &transport_secret,
                    b"app",
                    b"transport",
                )
                .unwrap(),
            vec![4, 2, 4, 2]
        );
        assert_eq!(
            guard.open_once(
                &envelope,
                &message_secret,
                &transport_secret,
                b"app",
                b"transport",
            ),
            Err(CryptoError::ReplayDetected)
        );
    }

    #[test]
    fn failed_authentication_does_not_poison_replay_state() {
        let message_secret = MessageSecret::random();
        let transport_secret = TransportSecret::random();
        let envelope = LayeredEnvelope::seal(
            &[7, 7, 7],
            &message_secret,
            &transport_secret,
            b"app",
            b"transport",
        )
        .unwrap();
        let mut guard = ReplayGuard::new(8);

        assert_eq!(
            guard.open_once(
                &envelope,
                &message_secret,
                &transport_secret,
                b"wrong-app",
                b"transport",
            ),
            Err(CryptoError::AuthenticationFailed)
        );
        assert_eq!(
            guard
                .open_once(
                    &envelope,
                    &message_secret,
                    &transport_secret,
                    b"app",
                    b"transport",
                )
                .unwrap(),
            vec![7, 7, 7]
        );
    }

    #[test]
    fn replay_window_is_bounded_and_evicts_oldest_entry() {
        let message_secret = MessageSecret::random();
        let transport_secret = TransportSecret::random();
        let first = LayeredEnvelope::seal(
            &[1],
            &message_secret,
            &transport_secret,
            b"app",
            b"transport",
        )
        .unwrap();
        let second = LayeredEnvelope::seal(
            &[2],
            &message_secret,
            &transport_secret,
            b"app",
            b"transport",
        )
        .unwrap();
        let mut guard = ReplayGuard::new(1);

        guard
            .open_once(
                &first,
                &message_secret,
                &transport_secret,
                b"app",
                b"transport",
            )
            .unwrap();
        guard
            .open_once(
                &second,
                &message_secret,
                &transport_secret,
                b"app",
                b"transport",
            )
            .unwrap();

        assert_eq!(
            guard
                .open_once(
                    &first,
                    &message_secret,
                    &transport_secret,
                    b"app",
                    b"transport",
                )
                .unwrap(),
            vec![1]
        );
    }

    #[test]
    fn fresh_envelope_with_same_plaintext_is_not_a_replay() {
        let message_secret = MessageSecret::random();
        let transport_secret = TransportSecret::random();
        let first = LayeredEnvelope::seal(
            &[1, 1, 2, 3],
            &message_secret,
            &transport_secret,
            b"app",
            b"transport",
        )
        .unwrap();
        let second = LayeredEnvelope::seal(
            &[1, 1, 2, 3],
            &message_secret,
            &transport_secret,
            b"app",
            b"transport",
        )
        .unwrap();
        let mut guard = ReplayGuard::default();

        guard
            .open_once(
                &first,
                &message_secret,
                &transport_secret,
                b"app",
                b"transport",
            )
            .unwrap();
        assert_eq!(
            guard
                .open_once(
                    &second,
                    &message_secret,
                    &transport_secret,
                    b"app",
                    b"transport",
                )
                .unwrap(),
            vec![1, 1, 2, 3]
        );
    }
}
