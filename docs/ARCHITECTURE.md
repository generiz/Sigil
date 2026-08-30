# Architecture

Sigil separates endpoint input hardening from cryptographic messaging.

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

## Rendering

The secure surface may render glyph geometry directly rather than delegating sensitive strings to OS text widgets. This reduces accidental exposure through accessibility text nodes, clipboard integrations and framework text caches.

Rendered pixels remain observable to a sufficiently privileged operating system or display-capture adversary.

## Cryptographic layer

End-to-end encryption is a separate layer. The intended direction is:

- authenticated device identity
- forward-secret session establishment
- ratcheted message keys
- AEAD-protected envelopes
- independent attachment keys
- hardware-backed long-term key material where available

No custom cipher will be introduced for the symbol layer.

## Platform split

The long-term implementation is expected to use:

- Kotlin for Android lifecycle and platform integration
- Rust for sensitive state machines and protocol logic
- a custom Android rendering surface for secure composition

The core must remain testable independently of the mobile UI.
