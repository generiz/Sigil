# Sigil

**Status: research prototype. Sigil is not a production messenger.**

Sigil is secure-messaging research focused on minimizing plaintext exposure, separating visible identity from cryptographic identity, and keeping network identifiers short-lived.

A browser demonstrator is available at https://nicolaspintos.com/sigil/. The Rust/WebAssembly core performs the symbol pipeline, layered authenticated encryption and Reed-Solomon recovery. The node topology shown in the demo is explicitly simulated.

Sigil does not use the operating system's normal text model as the intended sensitive-data path. Input, symbol representation, cryptography, identity, visual presentation, media handling and network transport are separate trust domains.

## Message representation

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

`SecureSymbolStream` is binary. It does not require Unicode, `String`, an IME or a standard text widget. The receiver can resolve one `SymbolId` at a time for custom glyph rendering without first materializing the whole message as operating-system text.

The symbol map is not encryption. Confidentiality comes from authenticated encryption.

## Layered encryption

The Rust core uses two independent XChaCha20-Poly1305 layers.

```text
SecureSymbolStream
      |
inner XChaCha20-Poly1305
      |  MessageSecret
      v
inner ciphertext
      |
outer XChaCha20-Poly1305
      |  TransportSecret
      v
wire envelope
```

Both layers use fresh nonces and authenticated associated data. The inner nonce and ciphertext are themselves protected by the outer layer.

Secret wrapper types zeroize their backing bytes on drop and use redacted debug formatting. The wire parser also rejects malformed or oversized message envelopes before deeper processing.

This is not a ratchet yet. Authenticated key exchange, forward secrecy and ratchet advancement remain protocol work.

See `docs/CRYPTO_PIPELINE.md`.

## Replay handling

The core includes a bounded in-memory `ReplayGuard` for exact authenticated envelope replays. A wire envelope is added to replay state only after successful authentication; an exact replay inside the configured window is rejected.

This is a hardening primitive, not the final session protocol. Persistent/session message numbering, ordering semantics and ratchet-integrated replay protection remain future work.

## Encrypted fragment layer

Sigil 0.3 can encode the already encrypted wire envelope into redundant opaque pieces.

```text
outer authenticated wire envelope
      |
optional traffic-size padding
      |
Reed-Solomon erasure coding
      |
opaque fragments
      |
node distribution
```

The default policy creates **20 fragments: 12 data + 8 parity**. Any valid 12 are enough to reconstruct the encrypted wire object, so eight pieces may be lost without losing the message.

Each network-facing piece has its own random 256-bit capability and opaque bytes. It does not expose its coding index, recipient name, contact alias or a permanent message identifier. The capability-to-index map stays in an endpoint `FragmentManifest`.

Reed-Solomon provides resilience, not encryption. The reconstructed object must still pass outer XChaCha20-Poly1305 authentication before any inner message state is exposed.

See `docs/FRAGMENTATION.md`.

## Identity without names

The secure UI does not require a real name, phone number or global username.

A contact may appear only as a local random alias or visual marker. Trust is pinned to cryptographic identity material verified out-of-band with a fingerprint or QR representation. If the verified identity key changes, the contact enters `KeyChanged` and must be verified again.

A color, alias or shape never authenticates a person.

See `docs/IDENTITY.md` and `docs/VISUAL_LAYER.md`.

## Ephemeral network state

The network model does not require a permanent mailbox identifier.

A delivery epoch contains fresh message, delivery and routing tokens. The node pool supports 2 to 1000 distinct nodes.

Fragment placement spreads pieces across the pool. With enough available nodes, each piece in a distribution round receives a different target. Small pools are reused in shuffled balanced passes.

```text
client
  |
entry / transit path
  |
independent encrypted pieces
  |-- store node A
  |-- store node B
  |-- store node C
  `-- ...
```

`Maximum` is the default privacy-policy target and enables token rotation, size-class padding, route rotation, batching and bounded-delay targets at the policy level. Live nodes, live mix scheduling and retrieval are not implemented yet.

Multiple nodes and fragment dispersal reduce concentration of encrypted state; they do not make traffic impossible to trace or defeat a global observer by definition.

See `docs/PRIVACY_NETWORK.md`.

## Photos and audio

Images are intended to be decoded to pixels, reconstructed without source EXIF/XMP/container metadata, then encrypted before transport.

Voice messages are intended to move from microphone samples to normalized audio frames and encrypted chunks without requiring a long-lived plaintext recording first.

Received media stays inside Sigil's protected path unless the user explicitly exports it.

See `docs/MEDIA.md`.

## Security boundary

Sigil does not claim that plaintext can never exist, keylogging is impossible, a compromised endpoint remains confidential, or traffic is impossible to trace.

A capable targeted adversary may combine telecom visibility, compromised infrastructure, endpoint exploits, seized devices, account compromise, supply-chain access and traffic correlation. Sigil does not claim that a specific government, intelligence service or law-enforcement body cannot investigate or compromise a user.

If the user can see a glyph, color, image or message, the device necessarily contains enough information at some stage to render those pixels. The goal is to minimize semantic plaintext lifetime and avoid unnecessary OS text surfaces.

See `SECURITY.md`, `docs/THREAT_MODEL.md` and `docs/STATE_LEVEL_THREAT_MODEL.md`.

## Current implementation

Implemented in the Rust core:

- randomized secure-composition primitives
- message-scoped binary symbol codes
- `SecureSymbolStream` with symbol-by-symbol decode
- layered XChaCha20-Poly1305 authenticated encryption
- independent message and transport secrets
- secret zeroization and redacted debug formatting
- bounded wire-envelope parsing
- bounded exact-envelope replay rejection after successful authentication
- 12-of-20 default Reed-Solomon fragment recovery
- random per-fragment capabilities
- endpoint-only reconstruction manifest model
- recovery with missing pieces
- BLAKE3 reconstruction consistency check before AEAD verification
- fragment target spreading across node pools from 2 to 1000 nodes
- end-to-end binary pipeline test through encryption, piece loss, reconstruction and symbol decode
- pseudonymous contact/trust-state models
- identity fingerprints
- ephemeral delivery/message/routing tokens
- traffic-size classes and privacy-policy targets
- local visual marker derivation
- ephemeral visual render epochs
- cross-platform Rust CI
- pinned Rust dependency resolution with `Cargo.lock`
- scheduled dependency advisory audit

Not implemented yet:

- authenticated key exchange and forward-secret message ratchet
- persistent/session replay numbering and ordering semantics
- hardware-backed production key storage
- Android secure input/rendering surface
- live distributed nodes, upload/retrieval or mix scheduling
- fragment TTL and ratchet-derived reconstruction state
- live media codecs and media encryption pipeline
- reproducible signed production releases
- independent cryptographic/application security audit

Features are considered implemented only when code, tests and documentation agree.

See `docs/ARCHITECTURE.md`, `docs/THREAT_MODEL.md` and `docs/ROADMAP.md`.
