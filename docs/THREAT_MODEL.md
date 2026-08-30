# Threat model

Sigil is designed to reduce avoidable plaintext exposure on a mobile endpoint while preserving a clear distinction between endpoint hardening and end-to-end cryptography.

## Protect against

- third-party or compromised keyboard applications observing sensitive input through the normal IME path
- accidental clipboard exposure
- autofill and predictive-text collection
- standard text-widget accessibility disclosure
- stable touch-coordinate meaning across composition sessions
- trivial plaintext string scanning of the secure composition buffer
- unnecessary retention of sensitive state after composition

## Out of scope for the secure composer alone

- a compromised Android kernel
- privileged malware inside the Sigil process
- malicious firmware or touchscreen controller
- an attacker that captures both rendered pixels and touch coordinates
- GPU/framebuffer capture by a privileged adversary
- physical cameras observing the screen and the user's hands

These require platform integrity, operational controls or a hardened device profile in addition to application design.

## Security principles

### No security through symbol substitution

Ephemeral symbol tokens and randomized key layouts are not encryption. They exist to reduce exposure and break stable representations at the endpoint.

Message confidentiality must come from established authenticated encryption and ratcheted session keys.

### No absolute claims

Sigil does not claim that plaintext can never exist, that keylogging is impossible, or that a compromised endpoint remains confidential.

If a human can read a message on a display, the endpoint necessarily contains enough information at some stage to render that message.

### Minimize, separate, destroy

Sensitive state should be:

1. minimized to the information required for the current operation
2. separated across components where practical
3. retained for the shortest useful lifetime
4. explicitly cleared where the language and platform permit

## Future platform requirements

A hardened deployment should additionally rely on verified boot, a locked bootloader, current security patches, restricted accessibility services, hardware-backed key storage and tight application permissions.
