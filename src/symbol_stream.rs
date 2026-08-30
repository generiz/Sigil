use crate::{composer::SymbolId, crypto::MessageSecret};
use blake3::Hasher;
use std::{error::Error, fmt};
use zeroize::Zeroize;

const SYMBOL_CODE_LEN: usize = 16;

pub struct SymbolMapKey([u8; 32]);

impl SymbolMapKey {
    pub fn from_message_secret(message_secret: &MessageSecret) -> Self {
        Self(message_secret.derive_subkey(b"sigil-symbol-map-v1"))
    }

    fn code_for(&self, symbol: SymbolId) -> [u8; SYMBOL_CODE_LEN] {
        let mut hasher = Hasher::new_keyed(&self.0);
        hasher.update(b"sigil-symbol-code-v1");
        hasher.update(&symbol.0.to_be_bytes());
        let digest = hasher.finalize();
        let mut code = [0u8; SYMBOL_CODE_LEN];
        code.copy_from_slice(&digest.as_bytes()[..SYMBOL_CODE_LEN]);
        code
    }
}

impl fmt::Debug for SymbolMapKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SymbolMapKey([REDACTED])")
    }
}

impl Drop for SymbolMapKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolStreamError {
    MalformedStream,
    UnknownSymbolCode,
    InvalidIndex,
}

impl fmt::Display for SymbolStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedStream => write!(f, "malformed secure symbol stream"),
            Self::UnknownSymbolCode => write!(f, "unknown symbol code"),
            Self::InvalidIndex => write!(f, "invalid symbol index"),
        }
    }
}

impl Error for SymbolStreamError {}

pub struct SecureSymbolStream {
    bytes: Vec<u8>,
}

impl fmt::Debug for SecureSymbolStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecureSymbolStream")
            .field("symbol_count", &self.symbol_count())
            .field("bytes", &"[REDACTED]")
            .finish()
    }
}

impl SecureSymbolStream {
    pub fn encode(symbols: &[SymbolId], key: &SymbolMapKey) -> Self {
        let mut bytes = Vec::with_capacity(symbols.len() * SYMBOL_CODE_LEN);
        for symbol in symbols {
            bytes.extend_from_slice(&key.code_for(*symbol));
        }
        Self { bytes }
    }

    pub fn from_encoded(bytes: Vec<u8>) -> Result<Self, SymbolStreamError> {
        if !bytes.len().is_multiple_of(SYMBOL_CODE_LEN) {
            return Err(SymbolStreamError::MalformedStream);
        }
        Ok(Self { bytes })
    }

    pub fn symbol_count(&self) -> usize {
        self.bytes.len() / SYMBOL_CODE_LEN
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn decode_symbol_at(
        &self,
        index: usize,
        alphabet: &[SymbolId],
        key: &SymbolMapKey,
    ) -> Result<SymbolId, SymbolStreamError> {
        if index >= self.symbol_count() {
            return Err(SymbolStreamError::InvalidIndex);
        }

        let offset = index * SYMBOL_CODE_LEN;
        let encoded = &self.bytes[offset..offset + SYMBOL_CODE_LEN];

        for symbol in alphabet {
            if key.code_for(*symbol).as_slice() == encoded {
                return Ok(*symbol);
            }
        }

        Err(SymbolStreamError::UnknownSymbolCode)
    }

    pub fn clear(&mut self) {
        self.bytes.zeroize();
        self.bytes.clear();
    }
}

impl Drop for SecureSymbolStream {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alphabet() -> Vec<SymbolId> {
        (1..=40).map(SymbolId).collect()
    }

    #[test]
    fn receiver_decodes_without_materializing_os_text() {
        let secret = MessageSecret::random();
        let key = SymbolMapKey::from_message_secret(&secret);
        let symbols = [SymbolId(8), SymbolId(15), SymbolId(12), SymbolId(1)];
        let stream = SecureSymbolStream::encode(&symbols, &key);
        let alphabet = alphabet();

        for (index, expected) in symbols.iter().enumerate() {
            assert_eq!(
                stream.decode_symbol_at(index, &alphabet, &key).unwrap(),
                *expected
            );
        }
    }

    #[test]
    fn same_symbol_changes_code_with_message_secret() {
        let first_secret = MessageSecret::random();
        let second_secret = MessageSecret::random();
        let first_key = SymbolMapKey::from_message_secret(&first_secret);
        let second_key = SymbolMapKey::from_message_secret(&second_secret);

        let first = SecureSymbolStream::encode(&[SymbolId(8)], &first_key);
        let second = SecureSymbolStream::encode(&[SymbolId(8)], &second_key);

        assert_ne!(first.as_bytes(), second.as_bytes());
    }

    #[test]
    fn encoded_stream_has_no_unicode_requirement() {
        let secret = MessageSecret::random();
        let key = SymbolMapKey::from_message_secret(&secret);
        let stream = SecureSymbolStream::encode(&[SymbolId(1), SymbolId(2)], &key);
        assert_eq!(stream.as_bytes().len(), 2 * SYMBOL_CODE_LEN);
    }

    #[test]
    fn symbol_state_debug_output_is_redacted() {
        let secret = MessageSecret::random();
        let key = SymbolMapKey::from_message_secret(&secret);
        let stream = SecureSymbolStream::encode(&[SymbolId(1), SymbolId(2)], &key);

        assert!(format!("{key:?}").contains("REDACTED"));
        assert!(format!("{stream:?}").contains("REDACTED"));
        assert!(!format!("{stream:?}").contains(&format!("{:?}", stream.as_bytes())));
    }
}
