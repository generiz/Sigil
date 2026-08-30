# Roadmap

## 0.1 — core models

Implemented:

- randomized key layout
- opaque `SymbolId` model
- ephemeral input tokens
- sensitive-buffer clearing
- pseudonymous contact/trust-state model
- identity fingerprint model
- media normalization/chunk-planning contracts
- node-role visibility contracts
- ephemeral message/delivery/routing tokens
- node-pool model from 2 to 1000 nodes
- distinct-node route selection
- traffic size classes and privacy-policy targets
- local visual marker derivation
- ephemeral visual render epochs
- cross-platform Rust CI

## 0.2 — binary symbol and layered crypto core

Implemented:

- message-scoped `SymbolMapKey`
- opaque 128-bit symbol codes
- binary `SecureSymbolStream`
- symbol-by-symbol receive decode without an OS text requirement
- inner XChaCha20-Poly1305 envelope
- independent outer XChaCha20-Poly1305 transport envelope
- fresh nonces for both layers
- authenticated associated data on both layers
- tamper-rejection tests
- independent message/transport secret tests

Still required before calling this a messaging protocol:

- authenticated key exchange
- forward-secret ratchet
- replay and ordering rules
- key lifecycle and recovery policy
- interoperability test vectors

## 0.3 — encrypted-piece resilience

Implemented:

- Reed-Solomon coding applied only after the outer authenticated envelope exists
- default 12 data + 8 parity fragments
- reconstruction from any valid threshold of 12 fragments
- recovery with eight missing pieces
- random coding-alignment padding
- random 256-bit capability per fragment
- network-facing fragments without exposed coding index
- endpoint `FragmentManifest` with capability-to-index mapping
- BLAKE3 reconstruction consistency check
- balanced fragment target spreading across node pools from 2 to 1000 nodes
- integration test from `SymbolId` through double AEAD, piece loss, reconstruction and back to `SymbolId`

Still required for live use:

- authenticated or ratchet-derived manifest synchronization
- per-fragment authentication for early rejection of corrupted pieces
- TTL/expiry and deletion rules
- upload/retrieval protocol
- retry and partial-delivery state
- live multi-node failure tests

## 0.4 — Android secure surface

- custom touch-driven composition surface
- no system IME for secure input
- no standard text widget for sensitive composition
- custom glyph geometry renderer
- receive path renders `SymbolId` directly rather than creating a normal message `String`
- local contact color/shape/pattern marker
- visual render-epoch rotation
- clipboard/autofill disabled on secure surfaces
- screen-capture restrictions where supported
- overlay/accessibility-risk handling

## 0.5 — identity and invitations

- real identity-key generation and storage
- hardware-backed identity-key adapter
- QR / out-of-band verification flow
- one-time rendezvous invitation protocol
- pinned trust persistence
- explicit `KeyChanged` UX
- visual marker bound to verified identity, never alias

## 0.6 — authenticated session protocol

- reviewed authenticated session establishment
- forward-secret message ratchet
- message-secret advancement
- transport-secret lifecycle
- replay/order rules
- delivery-token derivation/binding from authenticated session state
- fragment capability/manifest derivation where appropriate
- crash/restart state recovery without key reuse
- interoperability vectors

## 0.7 — live privacy transport

- dedicated transport client
- live entry/transit/store node protocol
- no permanent mailbox identifier
- split-knowledge metadata enforcement
- per-message or tightly bounded delivery/routing token rotation
- node-pool discovery and health state
- independent fragment upload/retrieval
- route rotation
- optional encrypted first-hop tunnel/VPN
- opaque push wakeups
- delivery/retry state
- measured traffic-correlation analysis

## 0.8 — maximum-privacy transport

- size-class padding integrated into wire transport before fragment coding
- delivery-window batching
- bounded randomized delay
- latency/battery/bandwidth measurements
- unlinkability tests for repeated delivery and fragment identifiers
- explicit documentation of what a global passive observer can still correlate

This phase targets reduced simple linkability, not guaranteed anonymity.

## 0.9 — private media path

- image decode to pixel buffer
- metadata-free canonical re-encode
- normalized voice capture and Opus encoding
- authenticated encrypted media chunks
- bounded decoder resources
- protected local media cache
- no automatic gallery/shared-storage export
- explicit user-controlled export

## Later

- multi-device identity/session model
- group messaging with a reviewed protocol such as MLS
- isolated link viewer outside the secure trust domain
- hardened Android deployment profile
- independent security review

Features are not considered implemented until code, tests and documentation agree.
