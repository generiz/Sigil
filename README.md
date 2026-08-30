# Sigil

Secure mobile messaging research focused on reducing plaintext exposure, minimizing metadata and separating visible identity from cryptographic identity.

Sigil is designed around a secure composition surface that does not use the operating system's normal text pipeline. Sensitive input is handled through direct touch events, randomized key placement, ephemeral symbol mappings and custom glyph rendering.

The project treats endpoint hardening, identity, encryption, media handling and network privacy as separate layers. None of them is presented as a substitute for the others.

## Design principles

- no system IME in the secure composer
- no standard OS text field for sensitive input
- no clipboard, autofill or predictive-text integration
- custom glyph rendering for the secure composition surface
- randomized keyboard layout per composition session
- ephemeral internal symbol representation
- minimal lifetime for sensitive buffers
- pseudonymous contacts by default: no real names required in the UI
- cryptographic identity verification independent of the visible alias
- end-to-end encryption as a separate protocol layer
- hardware-backed key storage where the platform supports it
- privacy-oriented multi-hop delivery with explicit metadata boundaries
- media normalization before encryption to avoid forwarding source metadata

## Identity without names

A Sigil contact is not identified by a human name, phone number or username in the secure UI.

The visible contact label is a random alias such as `A73F-19C2-6D04-8BE1`. The alias can rotate and has no security meaning by itself.

Trust is bound to cryptographic identity material. A contact can be verified by comparing a fingerprint or scanning a QR code over an independent channel. Once verified, the pinned identity key is the authority. If that key changes, Sigil must surface a security event and require re-verification.

This lets the interface remain pseudonymous while still answering the important question: "is this the same cryptographic identity I verified before?"

See `docs/IDENTITY.md`.

## Network model

Sigil does not need an embedded browser for messaging. The intended client uses its own transport stack.

The privacy-network direction is split-knowledge delivery:

```text
Device
  |
optional encrypted tunnel / VPN
  |
Entry relay
  |
Transit relay
  |
Opaque mailbox relay
```

The entry relay may know the client's network address but should not know the peer identity or message content. The mailbox relay may know an opaque mailbox token but should not know the client's original address. Message content remains end-to-end encrypted.

This design reduces metadata concentration; it does not make traffic correlation or endpoint tracing impossible.

See `docs/PRIVACY_NETWORK.md`.

## Photos and audio

Source media should not be forwarded byte-for-byte by default.

Images are intended to be decoded to pixels, stripped of source metadata, re-encoded into a canonical format, chunked and encrypted with an independent media key.

Voice messages are intended to flow from microphone samples to a normalized audio representation such as PCM/Opus frames, then into encrypted chunks without first requiring a long-lived plaintext file.

Received media should remain inside Sigil's protected storage/rendering path unless the user explicitly exports it.

See `docs/MEDIA.md`.

## Security boundary

Randomized layouts and ephemeral symbol mappings can reduce exposure to specific classes of input logging, memory string scanning and accidental OS text integration. They do not make a compromised operating system safe.

Multi-hop relays can reduce how much metadata a single service learns. They cannot guarantee that a sufficiently capable observer cannot correlate traffic.

Media normalization can remove ordinary file metadata. It cannot remove identifying visual or acoustic content from the media itself.

Sigil does not claim immunity to keyloggers, tracing, interception or endpoint compromise.

## Current implementation

The repository currently contains the secure composition core plus early protocol-domain models for pseudonymous identity, media transfer planning and relay knowledge boundaries.

Authenticated key generation, message ratcheting, AEAD envelopes, Android UI, live relays and real media codecs are not implemented yet. Those components will be added only with established constructions, explicit tests and clear security boundaries.

See `docs/ARCHITECTURE.md`, `docs/THREAT_MODEL.md` and `docs/ROADMAP.md`.
