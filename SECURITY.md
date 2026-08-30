# Security policy

Sigil is a research prototype, not a production messenger and not a tool that claims immunity from targeted state-level investigation.

The security goal is narrower and testable: reduce avoidable plaintext exposure, reject unauthenticated modification, minimize stable metadata, compartmentalize secrets and make implementation limits explicit.

## Browser demo boundary

The browser demonstrator is a single-process loopback test harness. It is not a confidential two-party channel.

Secrets are ephemeral session objects generated inside the process. There is no passphrase to guess. `MessageSecret` and `TransportSecret` are random 32-byte values. An attacker who can read process or WebAssembly memory during a live `DemoSession` may recover them. An attacker who only obtains network fragments does not obtain plaintext from those fragments.

The JavaScript origin already owns the input bytes passed into WASM. The public demo JSON is deliberately restricted to wire digest, fragment counts, reconstruction/authentication booleans and version; it does not return input bytes, receiver plaintext, decoded symbols, fragment capabilities or fragment payload digests.

The demo binds application AAD to an ordered sender/receiver identity context. This means a different bound identity context fails authentication. It does **not** authenticate where those identities came from, does not sign the envelope and does not perform authenticated key exchange.

There is no AKE or secret-distribution protocol yet. Any future two-party client must solve how peers establish authenticated session material; that mechanism becomes a primary practical attack surface.

The demo's transport AAD remains a fixed demonstrator label rather than negotiated session/route state. The core supports arbitrary transport AAD, but the browser demo must not imply that a real transport session is authenticated today.

The build and hosting origin are part of the trust boundary. Compromise of JavaScript, the `.wasm` artifact, a browser extension, the hosting origin or the release/build path can bypass the intended confidentiality boundary without breaking XChaCha20-Poly1305.

## Supported security work

Security changes should improve one or more of these boundaries:

- authenticated encryption and protocol binding
- replay and ordering controls
- secret lifetime and zeroization
- pseudonymous identity verification
- parser and resource bounds
- fragment integrity and recovery behavior
- endpoint data minimization
- build and dependency integrity
- metadata minimization without anonymity claims

## State-level adversaries

A capable targeted adversary may combine telecom visibility, compromised infrastructure, endpoint exploits, seized devices, account compromise, supply-chain access and traffic correlation.

Sigil does not claim protection after full compromise of the operating system, kernel, firmware, secure hardware, browser origin or the running Sigil process. It also does not claim that traffic is untraceable or that a particular government, intelligence service or law-enforcement body cannot investigate a user.

Security claims must be phrased in terms of a defined capability and threat boundary, not the identity of the adversary.

## Secret-handling rules

- secret-bearing types must not expose key bytes through `Debug`, logs or error strings
- derived keys should live in zeroizing buffers where practical
- plaintext and decoded symbol state should have bounded lifetime
- public demo telemetry must not return reconstructed plaintext
- unauthenticated data must not reach a renderer
- malformed or oversized network objects must be rejected before expensive processing
- replay state must be bounded; production session replay protection must survive the lifecycle required by the session protocol

The current in-memory `ReplayGuard` rejects exact authenticated envelope replays within a bounded process-local window. It is not a substitute for ratchet message numbers, persistent replay state or a complete session protocol. The browser demonstrator intentionally does not use it because its **Ver paso a paso** feature re-evaluates the same envelope.

## Cryptography

Sigil does not invent cryptographic primitives. Current envelope code uses XChaCha20-Poly1305 from the RustCrypto ecosystem. Reed-Solomon is used for availability only and is not encryption or authentication.

There is no password KDF because there is no password-based secret in the current design. AEAD subkeys are derived from random session secrets using domain-separated BLAKE3 keyed hashing.

Authenticated key exchange, forward-secret ratcheting, identity signatures bound to session establishment and production key lifecycle are not yet implemented and must not be implied by the current prototype.

## Reporting a vulnerability

Do not publish secrets, private keys, personal message data or exploit material containing third-party data in a public issue.

For a reproducible bug that does not expose sensitive data, open a GitHub issue with:

- affected commit/version
- minimal reproduction steps
- expected and observed behavior
- security boundary affected

For sensitive reports, use the contact methods published in `https://nicolaspintos.com/.well-known/security.txt`.

## Review status

Sigil has not received an independent cryptographic or application-security audit. Passing tests, CI, Clippy or dependency checks is not equivalent to an external security review.
