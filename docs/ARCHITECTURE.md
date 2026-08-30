# Architecture

Sigil separates secure input, pseudonymous identity, cryptography, visual presentation, media handling and network transport into distinct trust domains.

## Message path

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
Authenticated encrypted envelope   [roadmap]
    |
Session ratchet                     [roadmap]
    |
Delivery epoch
    |-- fresh routing token
    |-- fresh mailbox token
    |-- traffic size class
    |
Privacy network                     [live transport roadmap]
```

Receive-side presentation is separate:

```text
Encrypted envelope
    |
Authentication + ratchet            [roadmap]
    |
Verified identity
    |
Local visual marker
    |
Fresh visual render epoch
    |
Custom protected surface            [Android roadmap]
    |
Pixels
```

The UI can therefore give a verified contact a stable local visual anchor, such as a color/shape marker, while transport identifiers and rendering state rotate independently.

## Secure composition

The sensitive composition surface does not rely on the normal Android text stack. Touch coordinates resolve to randomized key slots and session-scoped symbol tokens. Sensitive buffers are explicitly cleared where the language/platform permit.

The operating system still owns the touchscreen driver, compositor and scheduler. Avoiding IME/text widgets reduces voluntary exposure; it does not make a compromised kernel safe.

## Ephemeral symbol layer

Symbol tokens are short-lived endpoint-hardening state, not encryption. Message confidentiality must come from authenticated encryption and ratcheted keys.

## Pseudonymous identity

Human identity and protocol identity are separate. The secure UI does not require real names, phone numbers or global usernames.

Trust is pinned to public identity material. Fingerprint/QR verification happens over an independent channel. A key change transitions the contact to `KeyChanged`; aliases or visual markers cannot override that warning.

See `IDENTITY.md`.

## Ephemeral visual layer

A verified contact may be represented locally by a minimal visual marker. The marker is derived from the verified identity key plus a device-local visual secret, so the UI does not need a server-side profile label to preserve local continuity.

The render path also uses fresh visual epochs with short-lived render tokens. The intent is to avoid stable application-level mappings where they are unnecessary, not to claim that a compromised display stack cannot see final pixels.

The final renderer may eventually use a custom GPU surface rather than standard text/color widgets for sensitive views. That Android implementation is not present yet.

See `VISUAL_LAYER.md`.

## Cryptographic layer

Planned E2EE components are deliberately separate from symbol, visual and alias layers:

- authenticated device identity
- forward-secret session establishment
- ratcheted message keys
- AEAD-protected envelopes
- independent media keys
- hardware-backed long-term key material where available

No custom cipher is used for visual markers, aliases or symbol substitution.

## Delivery epochs

A network delivery should not require a permanent recipient username or stable mailbox identifier.

The core models a fresh `DeliveryEpoch` containing independent random epoch, routing and mailbox tokens. The protocol target is to replace these at message boundaries or other tightly bounded scopes through authenticated ratchet/session state.

Current random tokens are model primitives. They are not yet wired to a live cryptographic session or relay network.

## Privacy network

The design target is split-knowledge routing:

```text
Client -> Entry relay -> Transit relay -> Mailbox relay
```

The entry relay may see the source connection but should not know the peer identity or mailbox token. The mailbox relay may see an opaque mailbox token but should not know the original source address. No relay is designed to receive plaintext.

An optional VPN/encrypted tunnel can protect the local first hop, but VPN use alone is not anonymity.

Traffic-size classes can hide exact small-message lengths. Higher-privacy operation may later add route rotation, batching and bounded randomized delivery delays. These mechanisms trade latency/bandwidth for reduced linkability; they do not defeat a global observer by definition.

See `PRIVACY_NETWORK.md`.

## Media path

```text
Image / microphone
    |
Media sanitizer
    |
Canonical pixels / audio frames
    |
Chunk planner
    |
Independent media key               [crypto roadmap]
    |
Authenticated encrypted chunks      [crypto roadmap]
    |
Privacy network
```

Images should be reconstructed from decoded pixels so source EXIF/XMP, thumbnails, filenames and container metadata are not forwarded by default. Voice messages should be encoded from normalized audio frames without requiring a long-lived plaintext file.

See `MEDIA.md`.

## Browser boundary

A general-purpose browser/WebView is not part of the secure messaging trust domain. Initial links open externally with explicit user action. Any future isolated viewer must not share conversation keys or sensitive buffers.

## Platform split

Expected direction:

- Kotlin for Android lifecycle/platform integration
- Rust for sensitive state machines and protocol logic
- custom Android rendering surface for secure composition and sensitive receive views
- hardware-backed key adapters where available
- isolated media decoders/encoders where practical

The core remains independently testable from the mobile UI.
