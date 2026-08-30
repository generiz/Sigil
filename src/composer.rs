use rand::{rngs::OsRng, seq::SliceRandom, RngCore};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use zeroize::Zeroize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EphemeralToken(u64);

impl EphemeralToken {
    pub fn expose(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposerError {
    EmptyAlphabet,
    DuplicateSymbol(SymbolId),
    InvalidSlot(usize),
}

impl fmt::Display for ComposerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAlphabet => write!(f, "symbol set cannot be empty"),
            Self::DuplicateSymbol(symbol) => write!(f, "duplicate symbol id {}", symbol.0),
            Self::InvalidSlot(slot) => write!(f, "invalid key slot {slot}"),
        }
    }
}

impl Error for ComposerError {}

#[derive(Debug)]
pub struct LayoutSession {
    slots: Vec<SymbolId>,
    tokens: Vec<(SymbolId, EphemeralToken)>,
}

impl LayoutSession {
    pub fn new(symbols: &[SymbolId]) -> Result<Self, ComposerError> {
        if symbols.is_empty() {
            return Err(ComposerError::EmptyAlphabet);
        }

        let mut seen = HashSet::with_capacity(symbols.len());
        for symbol in symbols {
            if !seen.insert(*symbol) {
                return Err(ComposerError::DuplicateSymbol(*symbol));
            }
        }

        let mut rng = OsRng;
        let mut slots = symbols.to_vec();
        slots.shuffle(&mut rng);

        let mut token_values = HashSet::with_capacity(symbols.len());
        let mut tokens = Vec::with_capacity(symbols.len());
        for symbol in symbols {
            let token = loop {
                let candidate = rng.next_u64();
                if candidate != 0 && token_values.insert(candidate) {
                    break EphemeralToken(candidate);
                }
            };
            tokens.push((*symbol, token));
        }

        Ok(Self { slots, tokens })
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn symbol_for_slot(&self, slot: usize) -> Result<SymbolId, ComposerError> {
        self.slots
            .get(slot)
            .copied()
            .ok_or(ComposerError::InvalidSlot(slot))
    }

    pub fn token_for_symbol(&self, symbol: SymbolId) -> Option<EphemeralToken> {
        self.tokens
            .iter()
            .find_map(|(candidate, token)| (*candidate == symbol).then_some(*token))
    }

    pub fn token_for_slot(&self, slot: usize) -> Result<EphemeralToken, ComposerError> {
        let symbol = self.symbol_for_slot(slot)?;
        self.token_for_symbol(symbol)
            .ok_or(ComposerError::InvalidSlot(slot))
    }
}

impl Drop for LayoutSession {
    fn drop(&mut self) {
        for (_, token) in &mut self.tokens {
            token.0.zeroize();
        }
        self.slots.clear();
        self.tokens.clear();
    }
}

#[derive(Debug, Default)]
pub struct SensitiveBuffer {
    tokens: Vec<u64>,
}

impl SensitiveBuffer {
    pub fn push(&mut self, token: EphemeralToken) {
        self.tokens.push(token.0);
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn as_tokens(&self) -> &[u64] {
        &self.tokens
    }

    pub fn clear(&mut self) {
        self.tokens.zeroize();
        self.tokens.clear();
    }
}

impl Drop for SensitiveBuffer {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alphabet() -> Vec<SymbolId> {
        (1..=36).map(SymbolId).collect()
    }

    #[test]
    fn layout_contains_every_symbol_exactly_once() {
        let symbols = alphabet();
        let session = LayoutSession::new(&symbols).unwrap();
        let mut actual: Vec<_> = (0..session.slot_count())
            .map(|slot| session.symbol_for_slot(slot).unwrap())
            .collect();
        actual.sort();
        assert_eq!(actual, symbols);
    }

    #[test]
    fn token_mapping_is_unique_within_session() {
        let symbols = alphabet();
        let session = LayoutSession::new(&symbols).unwrap();
        let tokens: HashSet<_> = symbols
            .iter()
            .map(|symbol| session.token_for_symbol(*symbol).unwrap().expose())
            .collect();
        assert_eq!(tokens.len(), symbols.len());
        assert!(!tokens.contains(&0));
    }

    #[test]
    fn slot_resolves_to_matching_ephemeral_token() {
        let symbols = alphabet();
        let session = LayoutSession::new(&symbols).unwrap();
        for slot in 0..session.slot_count() {
            let symbol = session.symbol_for_slot(slot).unwrap();
            assert_eq!(
                session.token_for_slot(slot).unwrap(),
                session.token_for_symbol(symbol).unwrap()
            );
        }
    }

    #[test]
    fn duplicate_symbols_are_rejected() {
        let result = LayoutSession::new(&[SymbolId(7), SymbolId(7)]);
        assert_eq!(
            result.unwrap_err(),
            ComposerError::DuplicateSymbol(SymbolId(7))
        );
    }

    #[test]
    fn sensitive_buffer_can_be_explicitly_cleared() {
        let session = LayoutSession::new(&alphabet()).unwrap();
        let mut buffer = SensitiveBuffer::default();
        buffer.push(session.token_for_slot(0).unwrap());
        buffer.push(session.token_for_slot(1).unwrap());
        assert_eq!(buffer.len(), 2);
        buffer.clear();
        assert!(buffer.is_empty());
    }
}
