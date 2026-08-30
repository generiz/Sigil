# Sigil

Secure mobile messaging research focused on reducing plaintext exposure, minimizing metadata and separating visible identity from cryptographic identity.

Sigil treats secure input, identity, encryption, visual presentation, media handling and network privacy as separate layers. None is presented as a substitute for the others.

## Core idea

The secure composer avoids the normal operating-system text path: no system IME, no standard text field, no clipboard/autofill integration for sensitive composition, randomized key placement, ephemeral symbol mappings and custom rendering.

The receive side follows the same principle. A contact can remain visually recognizable to the human while protocol and delivery identifiers rotate underneath it.

**Stable to the human, ephemeral to the network.**

A local contact marker may be a simple color/shape/pattern, for example a green dot. That marker is derived locally from verified identity material and a device-local visual secret. It is not sent as a network identity and has no authentication authority by itself.

Each visual render epoch uses fresh internal state. This does not hide final pixels from a compromised OS/GPU; it avoids designing stable application-level visual identifiers where they are unnecessary.

See `docs/VISUAL_LAYER.md`.

## Identity without names

The secure UI does not require real names, phone numbers or global usernames. Random aliases are presentation only.

Trust is pinned to cryptographic identity material and verified out-of-band with a fingerprint or QR representation. If a verified key changes, trust becomes `KeyChanged` and requires a new verification decision.

The important invariant is not "this contact is named Marcelo". It is "this is the same cryptographic identity I verified before".

See `docs/IDENTITY.md`.

## Ephemeral delivery

The privacy-network target is split-knowledge delivery:

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

A delivery epoch has fresh opaque message, routing and mailbox tokens. The protocol direction is to rotate these at message boundaries rather than expose a stable username or mailbox address.

Traffic-size classes are modeled so exact small-message length does not need to be exposed. The maximum-privacy design target additionally allows route rotation, batching and bounded randomized delivery delay, accepting higher latency and bandwidth cost.

These measures reduce linkability. They do not guarantee anonymity against a global observer capable of correlating both ends of traffic.

See `docs/PRIVACY_NETWORK.md`.

## Photos and audio

Images are intended to be decoded to pixels, reconstructed without source EXIF/XMP/container metadata, chunked and encrypted with an independent media key.

Voice messages are intended to move from microphone samples to normalized audio frames and encrypted chunks without requiring a long-lived plaintext recording first.

Received media stays inside Sigil's protected path unless the user explicitly exports it.

See `docs/MEDIA.md`.

## Security boundary

Sigil does not claim that plaintext can never exist, that a compromised endpoint remains confidential, or that an IP address becomes impossible to trace.

If the user can see a color, glyph, image or message, the device necessarily produces enough final display information to render it. Endpoint hardening reduces avoidable exposure; end-to-end cryptography protects message content; privacy routing reduces metadata concentration.

## Current implementation

The Rust core currently contains:

- randomized secure-composition primitives
- ephemeral symbol tokens and explicit buffer clearing
- pseudonymous contact and trust-state models
- identity fingerprints
- media normalization/chunk planning contracts
- split-knowledge relay visibility contracts
- fresh delivery-epoch, routing and mailbox tokens
- traffic-size classes and privacy-policy targets
- local visual marker derivation
- ephemeral visual render epochs

Authenticated session establishment, a reviewed ratchet, AEAD message envelopes, Android UI/GPU rendering, live relays, batching/delay transport and real media codecs are not implemented yet.

Features are only considered implemented when code, tests and documentation agree.

See `docs/ARCHITECTURE.md`, `docs/THREAT_MODEL.md` and `docs/ROADMAP.md`.
