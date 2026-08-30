//! Browser-only loopback demonstrator.
//!
//! This module is intentionally not a two-party messaging protocol. `DemoSession` creates
//! ephemeral message and transport secrets inside one WASM process, seals an envelope,
//! fragments it, reconstructs it, and authenticates it in the same process. There is no
//! authenticated key exchange or secret distribution here.
//!
//! The JavaScript origin already owns the input bytes passed to the constructor. During a live
//! session, `MessageSecret` and `TransportSecret` exist in WebAssembly memory until the session
//! is freed and dropped. An origin/process memory compromise is therefore outside the
//! confidentiality boundary of this demo.
//!
//! Public JSON intentionally contains only wire digest, fragment counts, reconstruction/auth
//! results, and version. It does not return input bytes, decoded symbols, receiver plaintext,
//! fragment capabilities, or payload digests.

use crate::{
    identity::build_identity_bound_application_aad, FragmentBundle, FragmentPolicy,
    IdentityPublicKey, LayeredEnvelope, MessageSecret, SecureSymbolStream, SymbolId, SymbolMapKey,
    TransportSecret,
};
use serde::Serialize;
use std::collections::BTreeSet;
use wasm_bindgen::prelude::*;
use zeroize::Zeroize;

const MAX_INPUT_BYTES: usize = 512;
const TRANSPORT_AAD: &[u8] = b"sigil-web-demo-transport-v1";

// These are demo context identifiers, not authenticated identity keys and not an AKE.
const DEMO_SENDER_IDENTITY: [u8; 32] = [0xA1; 32];
const DEMO_RECEIVER_IDENTITY: [u8; 32] = [0xB2; 32];

#[derive(Debug, Serialize)]
struct DemoResult {
    version: &'static str,
    outer_wire_digest: String,
    fragments_total: usize,
    fragments_required: usize,
    fragments_available: usize,
    fragments_lost: usize,
    reconstruction_matches: bool,
    outer_authenticated: bool,
    inner_authenticated: bool,
}

#[wasm_bindgen]
pub struct DemoSession {
    message_secret: MessageSecret,
    transport_secret: TransportSecret,
    application_aad: Vec<u8>,
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

        let sender = IdentityPublicKey::new(DEMO_SENDER_IDENTITY);
        let receiver = IdentityPublicKey::new(DEMO_RECEIVER_IDENTITY);
        let application_aad = build_identity_bound_application_aad(&sender, &receiver);

        let envelope = LayeredEnvelope::seal(
            symbol_stream.as_bytes(),
            &message_secret,
            &transport_secret,
            &application_aad,
            TRANSPORT_AAD,
        )
        .map_err(js_error)?;

        let wire = envelope.to_wire_bytes();
        let outer_wire_digest = hex(blake3::hash(&wire).as_bytes());
        let bundle = FragmentBundle::split(&wire, FragmentPolicy::default()).map_err(js_error)?;

        Ok(Self {
            message_secret,
            transport_secret,
            application_aad,
            wire,
            outer_wire_digest,
            bundle,
        })
    }

    pub fn run(&self, missing_slots_csv: &str) -> Result<String, JsValue> {
        let total = self.bundle.manifest().total_fragments();
        let required = self.bundle.manifest().required_fragments();
        let missing = parse_missing_slots(missing_slots_csv, total)?;

        let available = self
            .bundle
            .fragments()
            .iter()
            .enumerate()
            .filter(|(index, _)| !missing.contains(&(index + 1)))
            .map(|(_, fragment)| fragment.clone())
            .collect::<Vec<_>>();

        let mut reconstruction_matches = false;
        let mut outer_authenticated = false;
        let mut inner_authenticated = false;

        if available.len() >= required {
            if let Ok(reconstructed) = self.bundle.manifest().reconstruct(&available) {
                reconstruction_matches = reconstructed == self.wire;
                if reconstruction_matches {
                    if let Ok(received) = LayeredEnvelope::from_wire_bytes(&reconstructed) {
                        if let Ok(mut opened) = received.open(
                            &self.message_secret,
                            &self.transport_secret,
                            &self.application_aad,
                            TRANSPORT_AAD,
                        ) {
                            // `open` authenticates both layers before returning the inner bytes.
                            outer_authenticated = true;
                            inner_authenticated = true;
                            opened.zeroize();
                        }
                    }
                }
            }
        }

        let result = DemoResult {
            version: env!("CARGO_PKG_VERSION"),
            outer_wire_digest: self.outer_wire_digest.clone(),
            fragments_total: total,
            fragments_required: required,
            fragments_available: available.len(),
            fragments_lost: missing.len(),
            reconstruction_matches,
            outer_authenticated,
            inner_authenticated,
        };

        serde_json::to_string(&result).map_err(js_error)
    }
}

#[wasm_bindgen]
pub fn sigil_demo_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
