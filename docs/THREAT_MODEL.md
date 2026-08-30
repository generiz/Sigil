# Threat model

Sigil aims to reduce avoidable plaintext exposure and metadata while keeping endpoint hardening, identity verification, symbol representation, visual presentation, transport privacy and end-to-end cryptography separate.

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
- accidental forwarding of source image/audio container metadata

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

Double encryption is only useful because the keys belong to different trust boundaries. It must not be described as automatically "twice as secure".

The current core does not yet provide authenticated key exchange, a Double Ratchet-equivalent session protocol, replay protection or production key lifecycle management. Random `MessageSecret` and `TransportSecret` values are primitives used to build and test the envelope boundary.

## Visual adversaries

Visual markers and render-token rotation are not encryption.

A device-local marker can avoid sending a human-readable identity to the network and can remove the need for a stable server-side profile label. Fresh render epochs reduce stable internal state. Neither prevents a compromised display stack from observing final pixels.

A copied color, shape or alias must never create trust. Trust comes only from verified identity material.

## Network adversaries

Individual nodes may be curious or compromised.

The model separates source-network visibility from final delivery-token visibility and plaintext. Delivery and routing tokens are intended to rotate rather than expose a permanent username or mailbox handle.

This does not defeat a global passive observer. An observer monitoring both sides of routes may correlate timing, size, direction and volume. VPN use changes who sees the first hop; it does not eliminate traffic analysis.

Size padding hides exact lengths within a class but not timing, direction or the existence of traffic. Batching and bounded randomized delay may reduce some timing correlation at a latency/battery cost, but they are not an anonymity proof.

Sigil must not claim that an IP address is impossible to trace.

## Future distributed-piece delivery

A future redundancy/fragmentation layer may operate only on already authenticated ciphertext.

It must not split plaintext and treat dispersion as confidentiality.

Redundant pieces can improve resilience and reduce metadata concentration when different infrastructure handles different pieces, but they do not create cryptographic secrecy by themselves. Reconstruction metadata is sensitive because it can link pieces and must remain endpoint-controlled or be derived from authenticated session state.

This layer is not implemented yet.

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

Symbol indirection, random aliases, visual markers, token rotation, padding and future fragmentation are not substitutes for authenticated encryption.

### Encrypt before distributing

The required ordering is:

```text
semantic symbol state
  -> authenticated message encryption
  -> authenticated transport encryption
  -> optional padding
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
