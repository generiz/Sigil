# Sigil

Secure mobile messaging research focused on reducing plaintext exposure at the endpoint.

Sigil explores a mobile composition model where sensitive input does not use the operating system's normal text pipeline. The secure composer is designed around direct touch input, randomized key placement, ephemeral symbol mappings and custom glyph rendering.

The project treats this as an endpoint-hardening layer, not as a replacement for cryptography.

## Design goals

- no system IME in the secure composer
- no `EditText` / OS text field for sensitive input
- no clipboard, autofill or predictive-text integration
- custom glyph rendering for the secure composition surface
- randomized keyboard layout per composition session
- ephemeral internal symbol representation
- minimal lifetime for sensitive buffers
- end-to-end encryption as a separate protocol layer
- hardware-backed key storage where the platform supports it
- explicit threat model and security boundaries

## Security boundary

Randomized layouts and ephemeral symbol mappings can reduce exposure to specific classes of input logging, memory string scanning and accidental OS text integration. They do not make a compromised operating system safe.

A sufficiently privileged attacker that can observe both the display and touch stream, inspect application memory, compromise the kernel or capture the GPU output may still recover message content.

Sigil will not claim immunity to keyloggers, tracing, interception or endpoint compromise.

## Architecture direction

```text
Touch surface
    |
Secure composer
    |-- randomized visual layout
    |-- ephemeral symbol map
    |-- custom glyph renderer
    |
Sensitive message representation
    |
Authenticated encrypted envelope
    |
Session ratchet
    |
Transport / relay
```

The current repository starts with the secure composition core and its invariants. Cryptographic protocol components will be added only with established constructions and test vectors.

See `docs/ARCHITECTURE.md`, `docs/THREAT_MODEL.md` and `docs/ROADMAP.md`.
