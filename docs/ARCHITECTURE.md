# Architecture

Sigil separates secure input, symbol representation, pseudonymous identity, cryptography, visual presentation, media handling and network transport into distinct trust domains.

## Message path

```text
Touch surface
    |
randomized key slot
    |
SymbolId
    |
message-scoped symbol code
    |
SecureSymbolStream
    |
inner authenticated encryption
    |
outer authenticated transport envelope
    |
DeliveryEpoch
    |-- fresh delivery token
    |-- fresh routing token
    |
node route
```

The intended receive path is the inverse:

```text
wire envelope
    |
outer authentication/decryption
    |
inner authentication/decryption
    |
SecureSymbolStream
    |
decode one SymbolId at a time
    |
custom glyph renderer
    |
pixels
```

The receiver does not need to reconstruct a normal OS text string before rendering.

## Secure composition

The sensitive composition surface does not rely on the normal Android text stack.

Touch coordinates resolve to randomized key slots. Slots resolve to internal `SymbolId` values and short-lived input tokens. Clipboard, autofill, predictive text and a system IME are not part of the intended secure composition path.

The operating system still owns the touchscreen driver, process scheduler and final display pipeline. Avoiding the OS text stack reduces voluntary exposure; it does not make a compromised kernel safe.

## Message-scoped symbol representation

A symbol has no permanent transmitted code.

`SymbolMapKey` is derived from message secret material. The same internal symbol therefore receives a different 128-bit opaque code when the message secret changes.

`SecureSymbolStream` is a sequence of those binary codes. It has no Unicode or `String` requirement.

The receiver can resolve individual symbol codes against the same message-scoped map and pass the resulting `SymbolId` directly to the custom glyph layer.

This mapping is endpoint compartmentalization, not message encryption.

## Cryptographic layers

The current Rust core implements a two-layer authenticated envelope with XChaCha20-Poly1305.

```text
SecureSymbolStream
        |
        | MessageSecret
        v
inner XChaCha20-Poly1305
        |
        v
version + inner nonce + inner ciphertext
        |
        | TransportSecret
        v
outer XChaCha20-Poly1305
        |
        v
version + outer nonce + outer ciphertext
```

The inner and outer layers use independent secret domains and fresh nonces. The inner nonce is not exposed outside the outer authenticated envelope.

The intended security roles are different:

- inner layer: end-to-end message confidentiality/integrity
- outer layer: independent transport protection

This is not yet a full messaging protocol. Authenticated session establishment, forward secrecy, ratchet advancement, replay handling and key lifecycle rules remain roadmap work.

## Pseudonymous identity

Human identity and protocol identity remain separate.

The secure UI does not require real names, phone numbers or global usernames. Trust is pinned to verified cryptographic identity material. Fingerprint/QR verification happens through an independent channel.

If a pinned identity key changes, the contact enters `KeyChanged`. A matching alias, color or visual marker cannot override that state.

See `IDENTITY.md`.

## Ephemeral visual layer

A verified contact may be represented locally by a minimal visual marker derived from the verified identity key plus a device-local visual secret.

The human can therefore recognize a stable local marker while render tokens and network identifiers rotate independently.

Fresh render epochs reduce stable application-level state. They do not hide final pixels from a compromised OS/GPU.

See `VISUAL_LAYER.md`.

## Ephemeral delivery network

Sigil no longer requires a permanent mailbox identifier in its core model.

A fresh `DeliveryEpoch` contains:

```text
MessageEpoch
DeliveryToken
RoutingToken
```

These values are short-lived protocol state, not usernames.

The node model supports a pool of 2 to 1000 distinct nodes. An individual route selects a distinct subset from that pool.

```text
client -> entry -> [transit ...] -> store/delivery
```

The role boundaries are designed so that:

- entry can observe the incoming network connection but not the final delivery token
- transit does not require peer identity or plaintext
- store/delivery can handle an opaque delivery token without the original client address or plaintext

The default policy target is `Maximum`, preferring token rotation, padding, route rotation, batching and bounded-delay targets over minimum latency.

Live node transport, batching and delay scheduling are not implemented yet.

See `PRIVACY_NETWORK.md`.

## Future encrypted-piece delivery

A future distributed-delivery layer may take the already authenticated outer wire envelope and encode it into redundant pieces for resilience across multiple routes/nodes.

Ordering is mandatory:

```text
symbols
  -> inner AEAD
  -> outer AEAD
  -> padding
  -> redundancy/fragmentation
  -> distributed delivery
```

Sigil must never split plaintext and treat fragmentation as confidentiality.

The local endpoint would retain or derive the reconstruction metadata. Infrastructure should not require a permanent message identifier or human recipient label merely to hold a piece.

This phase is not implemented yet and is not an anonymity guarantee.

## Media path

```text
image / microphone
    |
controlled decoder/capture
    |
canonical pixels / audio frames
    |
media encryption
    |
transport
```

Images should be reconstructed from decoded pixels so source EXIF/XMP, thumbnails, filenames and container metadata are not forwarded by default.

Voice messages should be encoded from normalized audio frames without requiring a long-lived plaintext recording.

See `MEDIA.md`.

## Browser boundary

A general-purpose browser/WebView is not part of the secure messaging trust domain. Initial links open externally with explicit user action.

Any future isolated viewer must not share message secrets, transport secrets, symbol buffers or sensitive render state.

## Platform split

Expected direction:

- Kotlin for Android lifecycle/platform integration
- Rust for sensitive state machines and protocol logic
- custom Android rendering surface for secure input and receive views
- hardware-backed key adapters where available
- isolated media decoders/encoders where practical

The core remains independently testable from the mobile UI.
