use crate::{
    FragmentBundle, FragmentPolicy, LayeredEnvelope, MessageSecret, SecureSymbolStream, SymbolId,
    SymbolMapKey, TransportSecret,
};
use serde::Serialize;
use std::collections::BTreeSet;
use wasm_bindgen::prelude::*;

const MAX_INPUT_BYTES: usize = 512;
const SYMBOL_CODE_BYTES: usize = 16;
const APPLICATION_AAD: &[u8] = b"sigil-web-demo-application-v1";
const TRANSPORT_AAD: &[u8] = b"sigil-web-demo-transport-v1";

#[derive(Debug, Serialize)]
struct FragmentView {
    display_slot: usize,
    capability: String,
    payload_digest: String,
    bytes: usize,
    available: bool,
}

#[derive(Debug, Serialize)]
struct DemoResult {
    version: &'static str,
    input_bytes: usize,
    symbol_count: usize,
    symbol_codes: Vec<String>,
    symbol_stream_digest: String,
    outer_wire_bytes: usize,
    outer_wire_digest: String,
    fragments_total: usize,
    fragments_required: usize,
    fragments_available: usize,
    fragments_lost: usize,
    fragment_bytes: usize,
    fragments: Vec<FragmentView>,
    missing_slots: Vec<usize>,
    reconstruction_possible: bool,
    reconstructed_wire_digest: Option<String>,
    reconstruction_matches: bool,
    reconstruction_error: Option<String>,
    outer_authenticated: bool,
    inner_authenticated: bool,
    receiver_bytes: Option<Vec<u8>>,
    receiver_matches: bool,
}

#[wasm_bindgen]
pub struct DemoSession {
    input_bytes: Vec<u8>,
    message_secret: MessageSecret,
    transport_secret: TransportSecret,
    symbol_key: SymbolMapKey,
    symbol_count: usize,
    symbol_codes: Vec<String>,
    symbol_stream_digest: String,
    wire: Vec<u8>,
    outer_wire_digest: String,
    bundle: FragmentBundle,
}

fn js_error(value: impl ToString) -> JsValue {
    JsValue::from_str(&value.to_string())
}

fn hex(bytes: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(TABLE[(byte >> 4) as usize] as char);
        out.push(TABLE[(byte & 0x0f) as usize] as char);
    }
    out
}

fn short_hex(bytes: &[u8], take: usize) -> String {
    hex(&bytes[..bytes.len().min(take)])
}

fn parse_missing_slots(value: &str, total: usize) -> Result<BTreeSet<usize>, JsValue> {
    let mut slots = BTreeSet::new();
    for part in value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        let slot = part
            .parse::<usize>()
            .map_err(|_| js_error("missing fragment slots must be comma-separated integers"))?;
        if slot == 0 || slot > total {
            return Err(js_error(
                "missing fragment slot is outside this demo session",
            ));
        }
        slots.insert(slot);
    }
    Ok(slots)
}

#[wasm_bindgen]
impl DemoSession {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &[u8]) -> Result<DemoSession, JsValue> {
        if input.is_empty() {
            return Err(js_error("message cannot be empty"));
        }
        if input.len() > MAX_INPUT_BYTES {
            return Err(js_error("message exceeds 512-byte browser demo limit"));
        }

        let symbols: Vec<SymbolId> = input
            .iter()
            .map(|byte| SymbolId(u16::from(*byte) + 1))
            .collect();

        let message_secret = MessageSecret::random();
        let transport_secret = TransportSecret::random();
        let symbol_key = SymbolMapKey::from_message_secret(&message_secret);
        let symbol_stream = SecureSymbolStream::encode(&symbols, &symbol_key);

        let symbol_codes = symbol_stream
            .as_bytes()
            .chunks(SYMBOL_CODE_BYTES)
            .take(32)
            .map(hex)
            .collect::<Vec<_>>();
        let symbol_stream_digest = hex(blake3::hash(symbol_stream.as_bytes()).as_bytes());

        let envelope = LayeredEnvelope::seal(
            symbol_stream.as_bytes(),
            &message_secret,
            &transport_secret,
            APPLICATION_AAD,
            TRANSPORT_AAD,
        )
        .map_err(js_error)?;

        let wire = envelope.to_wire_bytes();
        let outer_wire_digest = hex(blake3::hash(&wire).as_bytes());
        let bundle = FragmentBundle::split(&wire, FragmentPolicy::default()).map_err(js_error)?;

        Ok(Self {
            input_bytes: input.to_vec(),
            message_secret,
            transport_secret,
            symbol_key,
            symbol_count: symbols.len(),
            symbol_codes,
            symbol_stream_digest,
            wire,
            outer_wire_digest,
            bundle,
        })
    }

    pub fn run(&self, missing_slots_csv: &str) -> Result<String, JsValue> {
        let total = self.bundle.manifest().total_fragments();
        let required = self.bundle.manifest().required_fragments();
        let missing = parse_missing_slots(missing_slots_csv, total)?;

        let fragments = self
            .bundle
            .fragments()
            .iter()
            .enumerate()
            .map(|(index, fragment)| FragmentView {
                display_slot: index + 1,
                capability: short_hex(fragment.capability().as_bytes(), 8),
                payload_digest: short_hex(blake3::hash(fragment.payload()).as_bytes(), 8),
                bytes: fragment.payload().len(),
                available: !missing.contains(&(index + 1)),
            })
            .collect::<Vec<_>>();

        let available = self
            .bundle
            .fragments()
            .iter()
            .enumerate()
            .filter(|(index, _)| !missing.contains(&(index + 1)))
            .map(|(_, fragment)| fragment.clone())
            .collect::<Vec<_>>();

        let reconstruction_possible = available.len() >= required;
        let mut reconstructed_wire_digest = None;
        let mut reconstruction_matches = false;
        let mut reconstruction_error = None;
        let mut outer_authenticated = false;
        let mut inner_authenticated = false;
        let mut receiver_bytes = None;
        let mut receiver_matches = false;

        if reconstruction_possible {
            match self.bundle.manifest().reconstruct(&available) {
                Ok(reconstructed) => {
                    reconstructed_wire_digest = Some(hex(blake3::hash(&reconstructed).as_bytes()));
                    reconstruction_matches = reconstructed == self.wire;

                    if reconstruction_matches {
                        match LayeredEnvelope::from_wire_bytes(&reconstructed) {
                            Ok(received) => match received.open(
                                &self.message_secret,
                                &self.transport_secret,
                                APPLICATION_AAD,
                                TRANSPORT_AAD,
                            ) {
                                Ok(opened) => {
                                    outer_authenticated = true;
                                    inner_authenticated = true;
                                    let incoming = SecureSymbolStream::from_encoded(opened)
                                        .map_err(js_error)?;
                                    let alphabet: Vec<SymbolId> = (1..=256).map(SymbolId).collect();
                                    let mut decoded = Vec::with_capacity(incoming.symbol_count());

                                    for index in 0..incoming.symbol_count() {
                                        let symbol = incoming
                                            .decode_symbol_at(index, &alphabet, &self.symbol_key)
                                            .map_err(js_error)?;
                                        let byte = symbol
                                            .0
                                            .checked_sub(1)
                                            .and_then(|value| u8::try_from(value).ok())
                                            .ok_or_else(|| js_error("decoded symbol is outside browser adapter alphabet"))?;
                                        decoded.push(byte);
                                    }

                                    receiver_matches = decoded == self.input_bytes;
                                    receiver_bytes = Some(decoded);
                                }
                                Err(error) => reconstruction_error = Some(error.to_string()),
                            },
                            Err(error) => reconstruction_error = Some(error.to_string()),
                        }
                    }
                }
                Err(error) => reconstruction_error = Some(error.to_string()),
            }
        } else {
            reconstruction_error = Some("not enough fragments to reconstruct".to_owned());
        }

        let result = DemoResult {
            version: env!("CARGO_PKG_VERSION"),
            input_bytes: self.input_bytes.len(),
            symbol_count: self.symbol_count,
            symbol_codes: self.symbol_codes.clone(),
            symbol_stream_digest: self.symbol_stream_digest.clone(),
            outer_wire_bytes: self.wire.len(),
            outer_wire_digest: self.outer_wire_digest.clone(),
            fragments_total: total,
            fragments_required: required,
            fragments_available: available.len(),
            fragments_lost: missing.len(),
            fragment_bytes: self.bundle.manifest().shard_len(),
            fragments,
            missing_slots: missing.into_iter().collect(),
            reconstruction_possible,
            reconstructed_wire_digest,
            reconstruction_matches,
            reconstruction_error,
            outer_authenticated,
            inner_authenticated,
            receiver_bytes,
            receiver_matches,
        };

        serde_json::to_string(&result).map_err(js_error)
    }
}

#[wasm_bindgen]
pub fn sigil_demo_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[wasm_bindgen]
pub fn run_protocol_demo(input: &str, requested_loss: u8) -> Result<String, JsValue> {
    let session = DemoSession::new(input.as_bytes())?;
    let loss = usize::from(requested_loss).min(FragmentPolicy::default().parity_shards());
    let slots = (1..=loss)
        .map(|slot| slot.to_string())
        .collect::<Vec<_>>()
        .join(",");
    session.run(&slots)
}
