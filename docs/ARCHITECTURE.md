# Architecture

Sigil separates secure input, symbol representation, pseudonymous identity, cryptography, visual presentation, fragment resilience, media handling and network transport into distinct trust domains.

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
optional traffic-size padding
    |
Reed-Solomon fragment coding
    |
opaque fragment capabilities
    |
node distribution
```

The receive path reverses this order:

```text
opaque fragments
    |
threshold reconstruction
    |
outer authenticated wire envelope
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

Touch coordinates resolve to randomized key slots. Slots resolve to internal `SymbolId` values and short-lived input tokens. Clipboard, autofill, predictive text and a system IME are not part of the intended secure path.

The operating system still owns the touchscreen driver, process scheduler and final display pipeline. Avoiding the OS text stack reduces voluntary exposure; it does not make a compromised kernel safe.

## Message-scoped symbol representation

`SymbolMapKey` is derived from message secret material. The same internal symbol therefore receives a different 128-bit opaque code when the message secret changes.

`SecureSymbolStream` is binary and has no Unicode or `String` requirement. The receiver can resolve individual symbol codes and pass resulting `SymbolId` values directly to a custom glyph layer.

This mapping is endpoint compartmentalization, not message encryption.

## Cryptographic layers

The Rust core implements two XChaCha20-Poly1305 layers with independent secret domains.

```text
SecureSymbolStream
        |
        | MessageSecret
        v
inner XChaCha20-Poly1305
        |
version + inner nonce + inner ciphertext
        |
        | TransportSecret
        v
outer XChaCha20-Poly1305
        |
version + outer nonce + outer ciphertext
```

The inner layer protects end-to-end message content. The outer layer provides an independent transport envelope. Both authenticate their associated data and use fresh nonces.

Authenticated session establishment, forward secrecy, ratchet advancement, replay handling and production key lifecycle rules remain roadmap work.

See `CRYPTO_PIPELINE.md`.

## Encrypted fragment layer

Fragmentation occurs only after the outer authenticated envelope exists.

The default `FragmentPolicy` uses 12 data shards and 8 parity shards. Any valid set of 12 of the resulting 20 pieces can reconstruct the encrypted wire object.

Before coding, the wire object is padded with random alignment bytes to an exact multiple of the data-shard count. This is coding alignment, not traffic-size privacy padding.

Each `OpaqueFragment` exposes only:

```text
256-bit random capability
opaque shard payload
```

The fragment does not expose its coding index. `FragmentManifest` remains endpoint state and maps capabilities to coding positions, stores the original encrypted length and carries a BLAKE3 reconstruction digest.

The digest is a consistency check. Cryptographic authority remains the outer AEAD authentication that follows reconstruction.

See `FRAGMENTATION.md`.

## Pseudonymous identity

Human identity and protocol identity remain separate.

The secure UI does not require real names, phone numbers or global usernames. Trust is pinned to verified cryptographic identity material. Fingerprint/QR verification happens through an independent channel.

If a pinned identity key changes, the contact enters `KeyChanged`. A matching alias, color or visual marker cannot override that state.

See `IDENTITY.md`.

## Ephemeral visual layer

A verified contact may be represented locally by a minimal visual marker derived from the verified identity key plus a device-local visual secret.

The human can recognize a stable local marker while render tokens and network identifiers rotate independently.

Fresh render epochs reduce stable application-level state. They do not hide final pixels from a compromised OS/GPU.

See `VISUAL_LAYER.md`.

## Ephemeral delivery network

Sigil does not require a permanent mailbox identifier in its core model.

A fresh `DeliveryEpoch` contains:

```text
MessageEpoch
DeliveryToken
RoutingToken
```

The node model supports a pool of 2 to 1000 distinct nodes.

`NodePool::targets_for_fragments()` spreads fragment targets across the available pool. If enough nodes exist, every fragment in that round receives a distinct target. If the pool is smaller, targets are reused only after shuffled passes through the pool.

Target placement and route construction are separate concerns. A future live transport still needs entry/transit routing, retries, retrieval, expiry and authenticated state synchronization.

The default policy target is `Maximum`, preferring token rotation, padding, route rotation, batching and bounded-delay targets over minimum latency.

Live node transport, batching and delay scheduling are not implemented yet.

See `PRIVACY_NETWORK.md`.

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
fragment/transport path
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
