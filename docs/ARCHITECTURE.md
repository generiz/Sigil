# Architecture

Sigil separates secure input, pseudonymous identity, cryptography, media handling and network transport into distinct trust domains.

```text
Touch surface
    |
Secure composer
    |
Ephemeral symbol layer
    |
Sensitive message representation
    |
Verified pseudonymous identity
    |
Authenticated encrypted envelope
    |
Session ratchet
    |
Privacy network
    |
Entry -> Transit -> Opaque mailbox
```

Media follows a parallel path:

```text
Image / microphone
    |
Media sanitizer
    |
Canonical pixels / audio frames
    |
Chunk planner
    |
Independent media key
    |
Authenticated encrypted chunks
    |
Privacy network
```

## Secure composition path

The sensitive composition surface does not rely on the normal Android text stack.

```text
Touch event
    |
Key slot
    |
Ephemeral symbol token
    |
Custom glyph renderer
    |
Sensitive composition buffer
```

The operating system still owns the touchscreen driver, compositor and process scheduler. Sigil's goal is narrower: avoid voluntarily feeding sensitive content into IME, clipboard, autofill, predictive text and standard text widgets.

## Randomized layout

A composition session generates a fresh permutation of visible key positions. Coordinates therefore do not have a stable character meaning across sessions.

The visual layout permutation and the internal symbol mapping are independent. Learning one does not directly reveal the other.

Randomized layout is an endpoint-hardening measure only. An attacker that simultaneously observes the rendered keyboard and the touch stream can correlate the two.

## Ephemeral symbol layer

Sensitive characters are represented internally by session-scoped tokens rather than stable Unicode values inside the secure composer.

The mapping is intentionally short-lived and is cleared when composition ends. Tokens are not encryption and are never treated as cryptographic protection.

## Pseudonymous identity

Human identity and protocol identity are deliberately separated.

The secure UI does not need to store or display real names, phone numbers or usernames. A contact is shown using a random alias with no cryptographic authority.

Trust is pinned to a public identity key. Verification happens out-of-band using a fingerprint or QR representation. The contact record stores the verified key state, not a claim such as "this is Marcelo".

If the pinned key changes, the state becomes `KeyChanged` and the conversation must not silently inherit the old trust decision.

The alias may rotate independently of the identity key. A new alias is not a new identity; a new identity key is.

See `IDENTITY.md`.

## Cryptographic layer

End-to-end encryption is a separate layer. The intended direction is:

- authenticated device identity
- forward-secret session establishment
- ratcheted message keys
- AEAD-protected envelopes
- independent attachment keys
- hardware-backed long-term key material where available

No custom cipher will be introduced for the symbol layer, alias layer or media layer.

## Privacy network

Sigil messaging does not require an embedded browser. Network delivery is handled by a dedicated transport client.

The design target is split-knowledge routing:

```text
Client -> Entry relay -> Transit relay -> Mailbox relay
```

Each relay gets only the metadata necessary for its role. The entry relay can observe the client's network connection but should not receive a peer identity or plaintext mailbox description. The mailbox relay handles an opaque mailbox token but should not receive the original client address. Transit relays forward opaque traffic between layers.

An optional VPN or encrypted tunnel may protect the local path to the entry relay, but a VPN alone is not treated as anonymity.

The network must tolerate relay rotation and avoid designing a single service that learns client address, peer identity and content simultaneously.

See `PRIVACY_NETWORK.md`.

## Media sanitizer

Sigil does not assume that source photos or audio files are safe to forward as-is.

Images should be decoded into pixels and re-encoded from that pixel representation. Source EXIF/XMP, thumbnails, filenames and other container metadata are discarded unless explicitly preserved by policy.

Voice messages should be encoded from normalized audio frames rather than uploaded as an arbitrary source container. The normal path should not require a persistent plaintext recording before encryption.

Media is chunked for retryable transfer. Chunk encryption and integrity protection belong to the cryptographic layer and are not provided by chunking itself.

See `MEDIA.md`.

## Link handling

The secure messenger should not embed a general-purpose browser inside the trusted messaging surface. A browser engine brings HTML, JavaScript, cookies, web storage, fingerprinting and a large parser/runtime attack surface into the application.

Initial link handling should use explicit external opening. A future isolated link viewer, if added, must run outside the secure composition and cryptographic state domains and must not share conversation secrets.

## Platform split

The long-term implementation is expected to use:

- Kotlin for Android lifecycle and platform integration
- Rust for sensitive state machines and protocol logic
- a custom Android rendering surface for secure composition
- platform-backed key adapters where available
- isolated media decoders/encoders where practical

The core must remain testable independently of the mobile UI.
