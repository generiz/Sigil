# Threat model

Sigil aims to reduce avoidable plaintext exposure and metadata while keeping endpoint hardening, identity verification, visual presentation, transport privacy and end-to-end cryptography separate.

## Designed to reduce

- exposure to third-party keyboards through the normal IME path
- clipboard, autofill and predictive-text leakage
- standard text-widget accessibility disclosure
- stable touch-coordinate meaning across composition sessions
- trivial plaintext string scanning of the secure composition buffer
- unnecessary retention of sensitive state
- casual real-name/phone exposure in the secure UI
- silent trust inheritance after an identity-key change
- stable application-level contact identifiers in delivery traffic
- exact small-message length leakage where padding classes are used
- one relay learning source network, peer identity and plaintext together
- accidental forwarding of source image/audio container metadata

## Endpoint limits

The secure composer and visual layer alone do not protect against:

- a compromised Android kernel
- privileged malware inside the Sigil process
- malicious firmware/touchscreen controller
- simultaneous capture of rendered pixels and touch coordinates
- privileged GPU/framebuffer capture
- physical cameras observing the screen or hands

If a human can read a message or recognize a color on the display, the endpoint necessarily contains enough information at some stage to render it.

## Visual adversaries

Visual markers and render-token rotation are not encryption.

A device-local marker can avoid sending a human-readable identity to the network and can remove the need for a stable server-side profile label. Fresh render epochs can reduce stable internal state. Neither prevents a compromised display stack from observing final pixels.

A copied color/shape/alias must never create trust. Trust comes only from verified identity material.

## Network adversaries

Individual relays may be curious or compromised. Split-knowledge routing aims to keep source network visibility separate from mailbox capability visibility and plaintext content.

Delivery epochs, routing tokens and mailbox tokens are intended to rotate rather than expose a permanent username/mailbox handle.

This does not defeat a global passive observer. An observer monitoring both sides of a route may correlate timing, size, direction and volume. VPN use changes who sees the first hop; it does not eliminate traffic analysis.

Size padding hides exact lengths within a class but not timing, direction or the existence of traffic. Batching and bounded randomized delay may reduce some timing correlation, at a latency/battery cost, but are not an anonymity proof.

Sigil must not claim that an IP address is impossible to trace.

## Identity adversaries

Random aliases and local visual markers are intentionally non-authoritative. They can be copied or imitated without creating trust.

Trust comes from fingerprint/QR verification over an independent channel followed by local pinning of cryptographic identity material.

If a verified identity key changes, Sigil must enter `KeyChanged` and require re-verification. Matching presentation state cannot suppress the warning.

Verification proves continuity with the key that was checked; it does not independently prove legal identity.

## Media adversaries

Media normalization can remove ordinary source-container metadata only when the decoder/re-encoder path is implemented correctly.

It cannot remove identifying information visible or audible in the content itself: faces, places, reflections, voices, background sounds or text may still identify a person/location.

Media decoders are parser attack surfaces and should be isolated and resource-bounded where practical.

## Security principles

### Encryption remains separate

Symbol substitution, random aliases, visual markers, token rotation and padding are not substitutes for authenticated encryption. Confidentiality must come from reviewed cryptographic constructions and ratcheted session keys.

### Stable to the human, ephemeral to the network

Human usability may require a stable local recognition anchor. That anchor should not become a permanent network identifier. Delivery state should be short-lived and cryptographically bound to authenticated session state once the protocol exists.

### Minimize relay knowledge

No relay role is designed to receive source network address, peer identity and plaintext together.

### Normalize before forwarding

Source media should be reconstructed into a controlled representation before encrypted transfer when practical. Byte-for-byte forwarding is not the privacy-preserving default.

### No absolute claims

Sigil does not claim that plaintext can never exist, keylogging is impossible, traffic is untraceable, or a compromised endpoint stays confidential.

### Minimize, separate, destroy

Sensitive state should be minimized, compartmentalized, retained for the shortest useful lifetime and explicitly cleared where the language/platform permit.

## Hardened platform direction

A hardened deployment should additionally use verified boot, locked bootloader, current patches, restricted accessibility services, hardware-backed key storage and tight application permissions.
