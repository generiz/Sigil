# Crypto pipeline

This document describes the current Rust-core message representation and encryption boundary.

## Principle

Sigil does not need to materialize sensitive message content as operating-system text in the intended native path.

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

The symbol layer is not encryption. Confidentiality comes from authenticated encryption.

## Browser demonstrator boundary

The browser demo is a **single-process loopback harness**. `DemoSession` generates `MessageSecret` and `TransportSecret`, seals the envelope, fragments it, reconstructs it and opens it inside the same WASM process.

`MessageSecret::random()` and `TransportSecret::random()` are secret-generation primitives. They are **not key agreement** and do not explain how two independent peers would establish compatible secrets.

There is no password, PIN or passphrase in this design. There is therefore no password to guess and no password KDF. AEAD subkeys are derived from random 32-byte session secrets with domain-separated BLAKE3 keyed hashing.

The JavaScript origin already owns the input bytes before WASM receives them. During a live `DemoSession`, message and transport secrets remain in WebAssembly memory until the object is freed/dropped. An attacker who can read the running origin/process/WASM memory may therefore recover plaintext or session secrets without attacking XChaCha20-Poly1305.

The public JSON returned by the demo is restricted to:

- version
- outer wire digest
- total / required / available / lost fragment counts
- `reconstruction_matches`
- `outer_authenticated`
- `inner_authenticated`

It does not return input bytes, receiver plaintext, decoded symbol bytes, fragment capabilities or fragment payload digests.

## Symbol map

`SymbolMapKey` is derived from `MessageSecret` using a domain-separated BLAKE3 keyed hash.

Each `SymbolId` is mapped to a 128-bit code for that message-secret context. A new message secret produces different codes for the same symbol.

This layer is not encryption. If an attacker controls the process or obtains the message secret while decoding, symbol indirection does not preserve confidentiality.

## Inner layer

The inner layer uses XChaCha20-Poly1305 with:

- a key derived from `MessageSecret`
- a fresh 192-bit nonce
- application associated data

Its plaintext is the binary `SecureSymbolStream`.

Its role is message confidentiality and integrity within the envelope primitive.

## Identity context binding

The core exposes `build_identity_bound_application_aad(sender, receiver)`. It builds ordered application AAD from sender and receiver identity fingerprints plus a domain-separated label.

The browser demo uses this AAD. Sealing under one sender/receiver context and opening under a different receiver context fails authentication.

This is **context binding only**. It is not:

- authenticated key exchange
- an identity signature
- proof that either identity key was obtained securely
- a ratchet

A future two-party protocol must authenticate identity material during session establishment rather than relying on local pinning alone.

## Outer layer

The outer layer also uses XChaCha20-Poly1305, but its key is derived from an independent `TransportSecret`.

Its plaintext is:

```text
version
inner nonce
inner ciphertext
```

The outer layer has its own fresh nonce and transport associated data.

The browser demo still uses a fixed transport-AAD label (`sigil-web-demo-transport-v1`). That is demonstrator debt, not negotiated route/session binding. The core already accepts arbitrary transport AAD.

The two AEAD layers exist for different trust domains; double encryption is not presented as an automatic doubling of security.

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
  -> SecureSymbolStream bytes
```

The browser demo zeroizes the opened inner byte buffer immediately after the loopback authentication result is established and does not export it through JSON.

A future native renderer may resolve symbols one at a time. The current browser demo no longer claims to demonstrate a confidential receiver renderer.

## Replay boundary

The core includes `ReplayGuard`, a bounded in-memory exact-wire replay cache. It inserts a digest only after successful authentication.

This is not a session replay protocol. It has no persistent message numbers, no ratchet state, no crash-safe replay history and no ordering semantics.

The browser demo intentionally does not use `ReplayGuard`: **Ver paso a paso** re-evaluates the same envelope by design. Restarting a process also creates new replay state.

## Fragmentation

Any redundancy or distributed-piece layer operates after authenticated encryption:

```text
symbols
  -> inner AEAD
  -> outer AEAD
  -> optional padding
  -> Reed-Solomon redundancy/fragmentation
  -> transport
```

The default 12-data + 8-parity policy reconstructs the encrypted wire object when at least 12 valid pieces remain. Reed-Solomon provides availability, not confidentiality or authentication. Final authority remains AEAD authentication.

The endpoint `FragmentManifest` links fragment capabilities to coding positions and is therefore linkability-sensitive endpoint state.

## Supply-chain boundary

The demo is delivered as JavaScript plus WebAssembly from the site origin. Compromising that origin, a browser extension, the generated `.wasm`, build credentials or the release path can be cheaper than attacking the cipher.

`Cargo.lock`, CI, Clippy and advisory scanning reduce avoidable dependency/build risk. They do not provide signed reproducible browser artifacts or independent verification of what the browser downloaded.

## What is not implemented

Still required for a real two-party secure-messaging protocol:

- authenticated key exchange / secret distribution
- identity signatures or equivalent authenticated session binding
- forward-secret ratchet
- persistent/session replay numbering
- message ordering policy
- crash-safe ratchet persistence
- production key storage / hardware-backed adapters
- authenticated release/update process
- reproducible signed releases
- interoperability vectors
- independent cryptographic/application-security review

The browser demo must not be described as a confidential two-party messenger until those boundaries are addressed and independently reviewed.
