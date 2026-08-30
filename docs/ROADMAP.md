# Roadmap

## 0.1 — secure composition core

- session-scoped randomized key layout
- opaque symbol identifiers
- ephemeral symbol-token mapping
- sensitive token buffer with explicit clearing
- deterministic invariants and fuzzable core boundaries
- cross-platform Rust CI

## 0.2 — Android secure surface

- custom touch-driven composition surface
- no system IME for secure input
- no standard text widget for sensitive composition
- custom glyph geometry renderer
- clipboard and autofill disabled on the secure surface
- screen-capture restrictions where supported
- overlay and accessibility-risk handling

## 0.3 — identity and encrypted envelopes

- device identity model
- hardware-backed key adapter
- authenticated session establishment
- forward-secret message ratchet
- AEAD message envelopes
- attachment encryption
- interoperability test vectors

## 0.4 — transport and delivery

- minimal relay/mailbox protocol
- sealed routing metadata where practical
- push notifications without message content
- multi-device session model
- delivery and retry state

## Later

- group messaging using a reviewed group protocol such as MLS
- optional privacy relay support
- hardened Android deployment profile
- independent security review

Features are not considered implemented until code, tests and documentation agree.
