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

## 0.3 — Android secure surface

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

## 0.4 — identity and invitations

- real identity-key generation and storage
- hardware-backed identity-key adapter
- QR / out-of-band verification flow
- one-time rendezvous invitation protocol
- pinned trust persistence
- explicit `KeyChanged` UX
- visual marker bound to verified identity, never alias

## 0.5 — authenticated session protocol

- reviewed authenticated session establishment
- forward-secret message ratchet
- message-secret advancement
- transport-secret lifecycle
- replay/order rules
- delivery-token derivation/binding from authenticated session state
- crash/restart state recovery without key reuse
- interoperability vectors

## 0.6 — live privacy transport

- dedicated transport client
- live entry/transit/store node protocol
- no permanent mailbox identifier
- split-knowledge metadata enforcement
- per-message or tightly bounded delivery/routing token rotation
- node-pool discovery and health state
- route rotation
- optional encrypted first-hop tunnel/VPN
- opaque push wakeups
- delivery/retry state
- measured traffic-correlation analysis

## 0.7 — maximum-privacy transport

- size-class padding integrated into wire transport
- delivery-window batching
- bounded randomized delay
- latency/battery/bandwidth measurements
- unlinkability tests for repeated delivery identifiers
- explicit documentation of what a global passive observer can still correlate

This phase targets reduced simple linkability, not guaranteed anonymity.

## 0.8 — encrypted-piece resilience

Research and implement only after the live encrypted transport is stable:

- redundancy/erasure coding applied to already authenticated ciphertext
- reconstruction with missing pieces
- endpoint-controlled or ratchet-derived reconstruction metadata
- independent delivery state for pieces
- expiry and deletion policy
- node-loss and partial-delivery tests
- measurement of storage/bandwidth overhead

Fragmentation is not encryption and must never operate on plaintext as a confidentiality mechanism.

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
