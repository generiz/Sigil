# Sigil

Secure mobile messaging research focused on minimizing plaintext exposure, separating visible identity from cryptographic identity, and keeping network identifiers short-lived.

Sigil does not use the operating system's normal text model as the intended sensitive-data path. The design treats input, symbol representation, cryptography, identity, visual presentation, media handling and network transport as separate trust domains.

**Stable to the human, ephemeral to the network.**

## Message representation

The secure path is designed around internal symbol identifiers rather than OS text strings.

```text
Touch
  |
randomized key slot
  |
SymbolId
  |
message-scoped opaque symbol code
  |
SecureSymbolStream
```

`SecureSymbolStream` is binary. It does not require Unicode, `String`, an IME or a standard text widget. A receiver with the same message-scoped symbol-map key can resolve one symbol at a time for custom glyph rendering without first materializing the whole message as operating-system text.

The symbol map is not encryption. Its purpose is to avoid a stable character representation inside the sensitive path. Confidentiality comes from authenticated encryption.

## Layered encryption

The Rust core now includes a layered authenticated-encryption primitive using XChaCha20-Poly1305.

```text
SecureSymbolStream
      |
inner XChaCha20-Poly1305
      |  message secret
      v
inner ciphertext
      |
outer XChaCha20-Poly1305
      |  independent transport secret
      v
wire envelope
```

The inner and outer layers use independently derived keys and fresh nonces. The inner nonce and inner ciphertext are themselves protected by the outer layer.

This is not a ratchet yet. `MessageSecret` and `TransportSecret` are core primitives; authenticated session establishment, forward secrecy and ratchet advancement remain separate protocol work.

## Identity without names

The secure UI does not require a real name, phone number or global username.

A contact may appear only as a local random alias or visual marker. Trust is pinned to cryptographic identity material verified out-of-band with a fingerprint or QR representation. If the verified identity key changes, the contact enters `KeyChanged` and must be verified again.

A color, alias or shape never authenticates a person.

See `docs/IDENTITY.md` and `docs/VISUAL_LAYER.md`.

## Ephemeral network state

The network model no longer requires a permanent mailbox identifier.

A delivery epoch contains fresh message, delivery and routing tokens. The node pool can model between 2 and 1000 distinct nodes, while an individual route selects only the nodes needed for that delivery.

```text
client
  |
entry node
  |
zero or more transit nodes
  |
store/delivery node
```

The entry role may observe the source connection but is not designed to receive the delivery token. A store role may observe an opaque delivery token but is not designed to receive the original client address, peer identity or plaintext.

`Maximum` is the default privacy policy target and enables token rotation, size-class padding, route rotation, batching and bounded-delay targets at the policy level. Live mix scheduling and live distributed transport are not implemented yet.

A future distributed-delivery phase may encode an already encrypted wire envelope into redundant pieces and place those pieces through independent routes. Fragmentation must happen after authenticated encryption, never on plaintext. Such coding improves resilience and may reduce metadata concentration, but it is not itself encryption or an anonymity proof.

See `docs/PRIVACY_NETWORK.md`.

## Photos and audio

Images are intended to be decoded to pixels, reconstructed without source EXIF/XMP/container metadata, then encrypted before transport.

Voice messages are intended to move from microphone samples to normalized audio frames and encrypted chunks without requiring a long-lived plaintext recording first.

Received media stays inside Sigil's protected path unless the user explicitly exports it.

See `docs/MEDIA.md`.

## Security boundary

Sigil does not claim that plaintext can never exist, keylogging is impossible, a compromised endpoint remains confidential, or traffic is impossible to trace.

If the user can see a glyph, color, image or message, the device necessarily contains enough information at some stage to render those pixels. The goal is to minimize semantic plaintext lifetime and avoid unnecessary OS text surfaces, not to deny that physical rendering exists.

Likewise, multiple relays, rotating tokens, padding or future fragment dispersal can reduce metadata concentration and simple linkability. They do not defeat a sufficiently capable global observer by definition.

## Current implementation

Implemented in the Rust core:

- randomized secure-composition primitives
- session-scoped input tokens and explicit buffer clearing
- message-scoped binary symbol codes
- `SecureSymbolStream` with symbol-by-symbol decode
- layered XChaCha20-Poly1305 authenticated encryption
- independent message and transport secrets
- pseudonymous contact/trust-state models
- identity fingerprints
- media normalization/chunk-planning contracts
- ephemeral delivery/message/routing tokens
- node-pool model from 2 to 1000 nodes
- route selection with distinct nodes
- traffic-size classes and privacy-policy targets
- local visual marker derivation
- ephemeral visual render epochs
- cross-platform Rust CI

Not implemented yet:

- authenticated key exchange
- forward-secret message ratchet
- hardware-backed production key storage
- Android secure input/rendering surface
- live distributed nodes or mix scheduling
- redundant fragment dispersal/recovery
- live media codecs and media encryption pipeline

Features are considered implemented only when code, tests and documentation agree.

See `docs/ARCHITECTURE.md`, `docs/THREAT_MODEL.md` and `docs/ROADMAP.md`.
