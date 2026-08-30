# Security policy

Sigil is a research prototype, not a production messenger and not a tool that claims immunity from targeted state-level investigation.

The security goal is narrower and testable: reduce avoidable plaintext exposure, reject unauthenticated modification, minimize stable metadata, compartmentalize secrets and make implementation limits explicit.

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

Sigil does not claim protection after full compromise of the operating system, kernel, firmware, secure hardware or the running Sigil process. It also does not claim that traffic is untraceable or that a particular government, intelligence service or law-enforcement body cannot investigate a user.

Security claims must be phrased in terms of a defined capability and threat boundary, not the identity of the adversary.

## Secret-handling rules

- secret-bearing types must not expose key bytes through `Debug`, logs or error strings
- derived keys should live in zeroizing buffers where practical
- plaintext and decoded symbol state should have bounded lifetime
- unauthenticated data must not reach the renderer
- malformed or oversized network objects must be rejected before expensive processing
- replay state must be bounded; production session replay protection must survive the lifecycle required by the session protocol

The current in-memory `ReplayGuard` rejects exact authenticated envelope replays within a bounded process-local window. It is not a substitute for ratchet message numbers, persistent replay state or a complete session protocol.

## Cryptography

Sigil does not invent cryptographic primitives. Current envelope code uses XChaCha20-Poly1305 from the RustCrypto ecosystem. Reed-Solomon is used for availability only and is not encryption or authentication.

Authenticated key exchange, forward-secret ratcheting and production key lifecycle are not yet implemented and must not be implied by the current prototype.

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
