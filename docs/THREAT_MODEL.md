# Threat model

Sigil aims to reduce avoidable plaintext exposure and metadata while keeping endpoint hardening, identity verification, symbol representation, visual presentation, fragment resilience, transport privacy and end-to-end cryptography separate.

For a capability-based model of a well-resourced targeted adversary, see [`STATE_LEVEL_THREAT_MODEL.md`](STATE_LEVEL_THREAT_MODEL.md). Sigil does not claim immunity from any specific government, intelligence service or law-enforcement body.

## Designed to reduce

- exposure to third-party keyboards through the normal IME path
- clipboard, autofill and predictive-text leakage
- standard text-widget accessibility disclosure
- stable touch-coordinate meaning across composition sessions
- stable transmitted character codes across messages
- trivial plaintext string scanning of the secure composition path
- unnecessary retention of sensitive state
- casual real-name/phone exposure in the secure UI
- silent trust inheritance after an identity-key change
- stable recipient identifiers in delivery traffic
- exact small-message length leakage where padding classes are used
- one node role learning source network, peer identity and plaintext together
- concentration of the full encrypted wire object on one storage node when fragment distribution is used
- loss of the encrypted object when a bounded number of fragment nodes are unavailable
- accidental forwarding of source image/audio container metadata
- accidental disclosure of secret wrapper contents through debug formatting
- exact authenticated envelope replay within a bounded process-local window
- unbounded envelope parsing by rejecting oversized wire objects before deeper processing

## Endpoint limits

The secure composer, symbol layer and visual layer alone do not protect against:

- a compromised Android kernel
- privileged malware inside the Sigil process
- malicious firmware or touchscreen controller
- simultaneous capture of rendered pixels and touch coordinates
- privileged GPU/framebuffer capture
- physical cameras observing the screen or hands

If a human can read a message or recognize a color on the display, the endpoint necessarily contains enough information at some stage to render it.

Sigil therefore minimizes semantic plaintext and standard OS text surfaces; it does not claim that meaningful information never exists at the endpoint.

## Symbol-layer adversaries

Message-scoped symbol codes are not a substitute for encryption.

They make a stable internal/transmitted character code unnecessary and allow the receive path to resolve one internal `SymbolId` at a time without first creating an OS text string.

If an attacker obtains the message secret or controls the process while decoding/rendering, symbol indirection does not preserve confidentiality.

## Cryptographic boundary

The current core uses two XChaCha20-Poly1305 layers with independent secret domains:

- inner message layer
- outer transport layer

Both layers authenticate their ciphertext and associated data.

Double encryption is useful here because the keys belong to different trust boundaries. It must not be described as automatically "twice as secure".

`MessageSecret` and `TransportSecret` zeroize their backing bytes on drop and use redacted `Debug` implementations so ordinary debug formatting does not print raw key material.

The envelope parser applies a bounded wire-size policy before deeper processing. This is a resource-safety boundary for message envelopes, not a limit for future attachment transport.

The current core includes a bounded in-memory `ReplayGuard` that records successfully authenticated wire-envelope digests and rejects an exact replay while it remains inside the configured window. Failed authentication is never inserted into replay state.

This is **not** yet the production replay protocol. Sigil still lacks authenticated key exchange, a Double Ratchet-equivalent session protocol, persistent/session message numbering, ordering semantics and production key lifecycle management. Random `MessageSecret` and `TransportSecret` values remain primitives used to build and test the envelope boundary.

## Fragment-layer adversaries

The fragment layer operates only on the already authenticated outer wire envelope.

The default 12-data + 8-parity policy can reconstruct the encrypted object from any valid 12 of 20 fragments. This protects availability against up to eight missing pieces.

It does not automatically correct maliciously modified shards. Reed-Solomon is an erasure code, not a Byzantine fault-tolerance protocol.

The endpoint manifest therefore checks a BLAKE3 digest after reconstruction, and the reconstructed wire object must still pass outer XChaCha20-Poly1305 authentication. Outer AEAD authentication remains the cryptographic authority.

A network-facing fragment exposes a random capability and opaque payload, but not its coding index. The endpoint manifest links capabilities back to positions. That manifest is therefore linkability-sensitive and must remain endpoint-controlled or be protected/derived by authenticated session state in the live protocol.

Fragment capabilities are locators, not identities and not substitutes for authenticated encryption.

Compromise of enough storage nodes may still allow an adversary to collect enough encrypted pieces to reconstruct the outer ciphertext. That does not reveal plaintext without the cryptographic keys, but it may reveal volume, timing and piece relationships if the adversary also observes routing metadata.

## Visual adversaries

Visual markers and render-token rotation are not encryption.

A device-local marker can avoid sending a human-readable identity to the network and can remove the need for a stable server-side profile label. Fresh render epochs reduce stable internal state. Neither prevents a compromised display stack from observing final pixels.

A copied color, shape or alias must never create trust. Trust comes only from verified identity material.

## Network adversaries

Individual nodes may be curious or compromised.

The model separates source-network visibility from final delivery-token visibility and plaintext. Delivery, routing and fragment capabilities are intended to rotate rather than expose a permanent username or mailbox handle.

This does not defeat a global passive observer. An observer monitoring both sides of routes may correlate timing, size, direction and volume. VPN use changes who sees the first hop; it does not eliminate traffic analysis.

Size padding hides exact lengths within a class but not timing, direction or the existence of traffic. Batching and bounded randomized delay may reduce some timing correlation at a latency/battery cost, but they are not an anonymity proof.

Sigil must not claim that an IP address is impossible to trace.

## Identity adversaries

Random aliases and local visual markers are intentionally non-authoritative. They can be copied or imitated without creating trust.

Trust comes from fingerprint/QR verification over an independent channel followed by local pinning of cryptographic identity material.

If a verified identity key changes, Sigil must enter `KeyChanged` and require re-verification. Matching presentation state cannot suppress the warning.

Verification proves continuity with the key that was checked; it does not independently prove legal identity.

## Media adversaries

Media normalization can remove ordinary source-container metadata only when the decoder/re-encoder path is implemented correctly.

It cannot remove identifying information visible or audible in the content itself: faces, places, reflections, voices, background sounds or text may still identify a person or location.

Media decoders are parser attack surfaces and should be isolated and resource-bounded where practical.

## Security principles

### Encryption remains separate

Symbol indirection, random aliases, visual markers, token rotation, padding and fragment dispersal are not substitutes for authenticated encryption.

### Encrypt before distributing

The required ordering is:

```text
semantic symbol state
  -> authenticated message encryption
  -> authenticated transport encryption
  -> optional traffic-size padding
  -> redundancy/fragmentation
  -> distributed transport
```

### Stable to the human, ephemeral to the network

Human usability may require a stable local recognition anchor. That anchor should not become a permanent network identifier.

### Minimize node knowledge

No node role is designed to receive source network address, peer identity and plaintext together.

### Normalize before forwarding

Source media should be reconstructed into a controlled representation before encrypted transfer when practical. Byte-for-byte forwarding is not the privacy-preserving default.

### No absolute claims

Sigil does not claim that plaintext can never exist, keylogging is impossible, traffic is untraceable, or a compromised endpoint stays confidential.

### Minimize, separate, destroy

Sensitive state should be minimized, compartmentalized, retained for the shortest useful lifetime and explicitly cleared where the language/platform permit.

## Hardened platform direction

A hardened deployment should additionally use verified boot, locked bootloader, current patches, restricted accessibility services, hardware-backed key storage and tight application permissions.

Build-chain hardening is also part of the threat model. CI now enforces formatting, tests and Clippy, pins the Rust dependency resolution with `Cargo.lock`, and runs an advisory audit against that resolution. These controls reduce avoidable supply-chain risk but are not a reproducible-build proof or an external security audit.
