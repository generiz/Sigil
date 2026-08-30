use crate::{
    FragmentBundle, FragmentPolicy, LayeredEnvelope, MessageSecret, SecureSymbolStream, SymbolId,
    SymbolMapKey, TransportSecret,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

const MAX_INPUT_BYTES: usize = 512;
const SYMBOL_CODE_BYTES: usize = 16;

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
    fragments_lost: usize,
    fragment_bytes: usize,
    fragments: Vec<FragmentView>,
    reconstructed_wire_digest: String,
    reconstruction_matches: bool,
    outer_authenticated: bool,
    inner_authenticated: bool,
    receiver_text: String,
    receiver_matches: bool,
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

#[wasm_bindgen]
pub fn sigil_demo_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[wasm_bindgen]
pub fn run_protocol_demo(input: &str, requested_loss: u8) -> Result<String, JsValue> {
    let input_bytes = input.as_bytes();
    if input_bytes.is_empty() {
        return Err(js_error("message cannot be empty"));
    }
    if input_bytes.len() > MAX_INPUT_BYTES {
        return Err(js_error("message exceeds 512-byte browser demo limit"));
    }

    // The browser adapter turns UTF-8 bytes into internal symbols. From this point until the
    // final adapter, the core operates on SymbolId values and binary buffers rather than text.
    let symbols: Vec<SymbolId> = input_bytes
        .iter()
        .map(|byte| SymbolId(u16::from(*byte) + 1))
        .collect();
    let alphabet: Vec<SymbolId> = (1..=256).map(SymbolId).collect();

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
        b"sigil-web-demo-application-v1",
        b"sigil-web-demo-transport-v1",
    )
    .map_err(js_error)?;

    let wire = envelope.to_wire_bytes();
    let outer_wire_digest = hex(blake3::hash(&wire).as_bytes());

    let policy = FragmentPolicy::default();
    let loss = usize::from(requested_loss).min(policy.parity_shards());
    let bundle = FragmentBundle::split(&wire, policy).map_err(js_error)?;

    let fragments = bundle
        .fragments()
        .iter()
        .enumerate()
        .map(|(index, fragment)| FragmentView {
            display_slot: index + 1,
            capability: short_hex(fragment.capability().as_bytes(), 8),
            payload_digest: short_hex(blake3::hash(fragment.payload()).as_bytes(), 8),
            bytes: fragment.payload().len(),
            available: index >= loss,
        })
        .collect::<Vec<_>>();

    let available = bundle.fragments()[loss..].to_vec();
    let reconstructed = bundle
        .manifest()
        .reconstruct(&available)
        .map_err(js_error)?;
    let reconstructed_wire_digest = hex(blake3::hash(&reconstructed).as_bytes());
    let reconstruction_matches = reconstructed == wire;

    let received = LayeredEnvelope::from_wire_bytes(&reconstructed).map_err(js_error)?;
    let opened = received
        .open(
            &message_secret,
            &transport_secret,
            b"sigil-web-demo-application-v1",
            b"sigil-web-demo-transport-v1",
        )
        .map_err(js_error)?;

    let incoming = SecureSymbolStream::from_encoded(opened).map_err(js_error)?;
    let mut decoded = Vec::with_capacity(incoming.symbol_count());
    for index in 0..incoming.symbol_count() {
        let symbol = incoming
            .decode_symbol_at(index, &alphabet, &symbol_key)
            .map_err(js_error)?;
        let byte = symbol
            .0
            .checked_sub(1)
            .and_then(|value| u8::try_from(value).ok())
            .ok_or_else(|| js_error("decoded symbol is outside browser adapter alphabet"))?;
        decoded.push(byte);
    }

    let receiver_text = String::from_utf8(decoded).map_err(js_error)?;
    let receiver_matches = receiver_text == input;

    let result = DemoResult {
        version: env!("CARGO_PKG_VERSION"),
        input_bytes: input_bytes.len(),
        symbol_count: symbols.len(),
        symbol_codes,
        symbol_stream_digest,
        outer_wire_bytes: wire.len(),
        outer_wire_digest,
        fragments_total: bundle.manifest().total_fragments(),
        fragments_required: bundle.manifest().required_fragments(),
        fragments_lost: loss,
        fragment_bytes: bundle.manifest().shard_len(),
        fragments,
        reconstructed_wire_digest,
        reconstruction_matches,
        outer_authenticated: true,
        inner_authenticated: true,
        receiver_text,
        receiver_matches,
    };

    serde_json::to_string(&result).map_err(js_error)
}
