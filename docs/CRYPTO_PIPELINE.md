# Crypto pipeline

This document describes the current Rust-core message representation and encryption boundary.

## Principle

Sigil does not need to materialize sensitive message content as operating-system text.

The intended path is:

```text
touch
  -> randomized slot
  -> SymbolId
  -> message-scoped opaque symbol code
  -> SecureSymbolStream
  -> inner authenticated encryption
  -> outer authenticated transport encryption
  -> wire bytes
```

The reverse path resolves individual symbols for the renderer rather than first creating a normal text string.

## Symbol map

`SymbolMapKey` is derived from `MessageSecret` using a domain-separated BLAKE3 keyed hash.

Each `SymbolId` is mapped to a 128-bit code for that message-secret context. A new message secret produces different codes for the same symbol.

This layer is not encryption. It prevents a stable character representation from being a required part of the sensitive pipeline.

## Inner layer

The inner layer uses XChaCha20-Poly1305 with:

- a key derived from `MessageSecret`
- a fresh 192-bit nonce
- application associated data

Its plaintext is the binary `SecureSymbolStream`.

Its role is message confidentiality and integrity.

## Outer layer

The outer layer also uses XChaCha20-Poly1305, but its key is derived from an independent `TransportSecret`.

Its plaintext is:

```text
version
inner nonce
inner ciphertext
```

The outer layer has its own fresh nonce and transport associated data.

Its role is transport-boundary protection. The two layers exist for different trust domains; double encryption is not presented as an automatic doubling of security.

## Wire representation

The current outer wire object is:

```text
version
outer nonce
outer ciphertext
```

The inner nonce is therefore not visible until the outer layer authenticates and decrypts successfully.

## Receive path

```text
wire bytes
  -> parse outer envelope
  -> authenticate/decrypt outer layer
  -> authenticate/decrypt inner layer
  -> SecureSymbolStream
  -> decode SymbolId at requested position
  -> custom glyph renderer
```

Authenticated decryption happens before symbol decoding.

## What is not implemented

This core primitive is not a complete secure-messaging protocol.

Still required:

- authenticated key exchange
- identity-key binding to session establishment
- forward-secret ratchet
- replay protection
- message ordering policy
- crash-safe ratchet persistence
- production key storage/hardware-backed adapters
- interoperability vectors

`MessageSecret::random()` and `TransportSecret::random()` are useful for primitive tests. A production client must obtain them from reviewed session protocols and key lifecycle rules.

## Distribution order

Any future redundancy or distributed-piece layer must operate after authenticated encryption:

```text
symbols
  -> inner AEAD
  -> outer AEAD
  -> padding
  -> redundancy/fragmentation
  -> transport
```

Fragmentation is never a confidentiality mechanism.
