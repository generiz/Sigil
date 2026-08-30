# Threat model

Sigil is designed to reduce avoidable plaintext exposure on a mobile endpoint, minimize unnecessary metadata and preserve a clear distinction between endpoint hardening, identity verification, transport privacy and end-to-end cryptography.

## Protect against

- third-party or compromised keyboard applications observing sensitive input through the normal IME path
- accidental clipboard exposure
- autofill and predictive-text collection
- standard text-widget accessibility disclosure
- stable touch-coordinate meaning across composition sessions
- trivial plaintext string scanning of the secure composition buffer
- unnecessary retention of sensitive state after composition
- casual exposure of real-world names in the secure contact UI
- a single relay learning client network address, peer identity and message content at the same time
- source photo metadata such as EXIF/XMP being forwarded accidentally
- source audio/container metadata being forwarded accidentally
- silent trust inheritance after a verified contact identity key changes

## Out of scope for the secure composer alone

- a compromised Android kernel
- privileged malware inside the Sigil process
- malicious firmware or touchscreen controller
- an attacker that captures both rendered pixels and touch coordinates
- GPU/framebuffer capture by a privileged adversary
- physical cameras observing the screen and the user's hands

These require platform integrity, operational controls or a hardened device profile in addition to application design.

## Network adversaries

The privacy-network design assumes that individual relays may be curious or compromised.

The architecture therefore aims to prevent any one relay from learning the complete relationship between source network address, destination mailbox and plaintext content.

It does not assume that relay separation defeats a global passive observer. An observer capable of monitoring both sides of a route may correlate timing, size, direction and volume. VPN use changes which network operator sees the first hop; it does not remove the possibility of traffic analysis.

Sigil must not claim that an IP address is impossible to trace or that multi-hop routing provides guaranteed anonymity.

## Identity adversaries

Visible aliases are intentionally non-authoritative.

A malicious party may copy, guess or imitate another contact's alias. That must not create trust.

Trust comes from verified cryptographic identity material. The expected workflow is fingerprint comparison or QR verification over an independent channel, followed by local pinning of the verified identity key.

If the identity key changes, Sigil must surface that state and require a new verification decision. A display alias must never override a key-change warning.

Verification also has a social boundary: confirming a key only proves continuity with the key that was verified. It does not independently prove a person's legal name or real-world biography.

## Media adversaries

Media normalization removes ordinary source-container metadata only when the decoder/re-encoder path is implemented correctly.

It does not remove identifying information contained in the visible image or audible recording itself. Faces, locations, reflections, voices, background sounds and visual text may still identify a person or place.

Media decoders are also a parser attack surface. They should be isolated where practical, bounded in memory/size and treated as untrusted-input processors.

## Security principles

### No security through symbol substitution

Ephemeral symbol tokens and randomized key layouts are not encryption. They exist to reduce exposure and break stable representations at the endpoint.

Message confidentiality must come from established authenticated encryption and ratcheted session keys.

### No security through alias secrecy

Random aliases are a UI privacy feature. They are not credentials, authentication factors or encryption keys.

### Minimize relay knowledge

A relay should receive only the routing metadata needed for its role. No single backend component should be designed to know source address, peer identity and plaintext content together.

### Normalize before forwarding

Source media should be decoded and reconstructed into a controlled representation before encrypted transfer when practical. Byte-for-byte forwarding is opt-in, not the privacy-preserving default.

### No absolute claims

Sigil does not claim that plaintext can never exist, that keylogging is impossible, that an IP address can never be traced, or that a compromised endpoint remains confidential.

If a human can read a message on a display, the endpoint necessarily contains enough information at some stage to render that message.

### Minimize, separate, destroy

Sensitive state should be:

1. minimized to the information required for the current operation
2. separated across components where practical
3. retained for the shortest useful lifetime
4. explicitly cleared where the language and platform permit

## Future platform requirements

A hardened deployment should additionally rely on verified boot, a locked bootloader, current security patches, restricted accessibility services, hardware-backed key storage and tight application permissions.
