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

## 0.3 — pseudonymous identity

- no required real names, phone numbers or usernames in the secure UI
- random contact aliases with independent rotation
- public identity-key fingerprint model
- QR / out-of-band verification flow
- pinned trust state
- explicit `KeyChanged` state requiring re-verification
- one-time invitation / rendezvous token design
- hardware-backed identity-key adapter

## 0.4 — authenticated encrypted envelopes

- authenticated session establishment
- forward-secret message ratchet
- AEAD message envelopes
- independent attachment/media keys
- replay and ordering rules
- interoperability test vectors

## 0.5 — privacy network

- dedicated transport client; no embedded browser dependency
- entry / transit / mailbox relay roles
- split-knowledge metadata model
- opaque mailbox tokens
- relay rotation
- optional encrypted tunnel or VPN to the entry relay
- push notifications without message content
- delivery and retry state
- traffic-correlation limits documented and tested where measurable

## 0.6 — private media path

- image decode to pixel buffer
- metadata-free canonical re-encode
- normalized voice-message capture and Opus encoding
- encrypted chunk transfer with independent media keys
- bounded decoder resources
- protected local media cache
- no automatic export to gallery or shared storage
- explicit user-controlled export

## Later

- multi-device identity and session model
- group messaging using a reviewed group protocol such as MLS
- isolated link viewer, separate from the secure messaging trust domain
- hardened Android deployment profile
- independent security review

Features are not considered implemented until code, tests and documentation agree.
