# Roadmap

## 0.1 — core models

Implemented:

- session-scoped randomized key layout
- opaque symbol identifiers and ephemeral symbol tokens
- sensitive buffer clearing
- pseudonymous contact/trust-state model
- identity fingerprint model
- media normalization/chunk planning contracts
- split-knowledge relay-role contracts
- opaque mailbox/routing/message epoch tokens
- traffic size classes and privacy-policy targets
- local visual marker derivation
- ephemeral visual render epochs
- cross-platform Rust CI

## 0.2 — Android secure surface

- custom touch-driven composition surface
- no system IME for secure input
- no standard text widget for sensitive composition
- custom glyph geometry renderer
- sensitive receive surface using the same renderer boundary
- local contact color/shape/pattern marker
- visual render-epoch rotation
- clipboard/autofill disabled on secure surfaces
- screen-capture restrictions where supported
- overlay/accessibility-risk handling

## 0.3 — identity and invitations

- real key generation and storage model
- hardware-backed identity-key adapter
- QR / out-of-band verification flow
- one-time rendezvous invitation protocol
- pinned trust persistence
- explicit `KeyChanged` UX
- visual marker bound to verified identity, never alias

## 0.4 — authenticated encrypted envelopes

- reviewed authenticated session establishment
- forward-secret message ratchet
- AEAD message envelopes
- independent media keys
- replay/order rules
- delivery-token derivation/binding from authenticated session state
- interoperability test vectors

## 0.5 — privacy network

- dedicated transport client
- live entry/transit/mailbox relay protocol
- split-knowledge metadata enforcement
- per-message/bounded-scope mailbox and routing token rotation
- route rotation
- optional encrypted tunnel/VPN to entry
- opaque push wakeups
- delivery/retry state
- measured traffic-correlation analysis

## 0.6 — maximum-privacy transport

- size-class padding integrated into envelope transport
- delivery-window batching
- bounded randomized delay
- latency/battery/bandwidth measurements
- unlinkability tests for repeated delivery identifiers
- explicit documentation of what a global passive observer can still correlate

This phase targets reduced linkability, not guaranteed anonymity.

## 0.7 — private media path

- image decode to pixel buffer
- metadata-free canonical re-encode
- normalized voice capture and Opus encoding
- authenticated encrypted chunk transfer
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
