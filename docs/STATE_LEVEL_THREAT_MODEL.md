# Capable targeted adversary model

This document describes how Sigil should be evaluated against a well-resourced, targeted adversary. It does not claim resistance to any specific government, intelligence service or law-enforcement body.

The useful question is not "who is the attacker?" but "what can the attacker observe or control?"

## Capability matrix

| Adversary capability | Sigil objective | Current status |
| --- | --- | --- |
| passive observation of one network segment | content remains authenticated ciphertext | implemented for the envelope core |
| compromised storage/relay node | node receives opaque encrypted fragments rather than plaintext | modeled; live relays not implemented |
| modification of encrypted traffic | reject before symbol rendering | implemented in AEAD tests |
| exact envelope replay | reject within a bounded local replay window | implemented as `ReplayGuard`; not yet a complete session replay protocol |
| loss of some fragment nodes | recover encrypted object when threshold remains available | implemented in Reed-Solomon core |
| maliciously oversized wire object | reject before unbounded allocation/processing | bounded in the envelope parser |
| accidental logging of secret wrapper types | do not print raw key material through `Debug` | implemented with redacted secret formatting |
| stolen encrypted network fragments | no plaintext without endpoint cryptographic state | envelope core implemented; production key lifecycle pending |
| observation of both ends of a route | reduce linkability where practical | not solved; timing/volume correlation remains |
| compromised relay infrastructure at scale | avoid giving one role plaintext + identity + source network by design | architectural objective; live network not implemented |
| compromised normal keyboard/clipboard path | avoid those text surfaces in the intended secure composer | prototype model/demo only; Android implementation pending |
| seized locked device | rely on platform encryption and hardware-backed keys | future platform integration |
| seized unlocked device | minimize retained plaintext/secrets | partial design only; confidentiality cannot be guaranteed |
| malicious app with accessibility/overlay privileges | reduce exposed text semantics and block risky input states | future Android work |
| Sigil process compromise | no reliable confidentiality guarantee | out of scope |
| kernel/firmware/baseband compromise | no reliable confidentiality guarantee | out of scope |
| malicious build/toolchain dependency | reduce risk through review, CI and dependency advisory checks | partially implemented; reproducible signed releases remain future work |

## What stronger adversaries change

A targeted attacker will normally attack the cheapest boundary rather than the strongest primitive. If XChaCha20-Poly1305 is correctly used, attacking the endpoint, key lifecycle, update path, build chain, user verification flow or network metadata may be far easier than attacking the cipher.

For that reason, adding more encryption layers is not the primary hardening path. Priority work is:

1. authenticated identity keys and signed invitation material
2. forward-secret session establishment and ratcheting with replay/order semantics
3. hardware-backed key adapter contracts
4. strict parser/resource bounds and fuzzing
5. authenticated update/release process and reproducible-build work
6. Android process/input/render isolation
7. independent cryptographic and application-security review
8. live relay implementation with measured metadata behavior

## Network correlation

Fragmentation, route rotation, padding, batching and delay can reduce some metadata concentration. They do not prove anonymity against an observer with broad visibility across ingress and egress.

Sigil must not claim that an IP address, device or communicating pair is impossible to correlate.

## Endpoint compromise

When the endpoint is fully compromised while the user is reading or composing a message, the attacker may observe the same semantic information the legitimate user can observe. Custom glyph rendering, symbol indirection and zeroization reduce common exposure paths but do not defeat privileged control of the device.

## Build-chain compromise

A secure protocol delivered through a malicious binary is not secure. Production readiness therefore requires more than protocol tests:

- pinned/reviewed dependencies
- advisory scanning
- minimal build permissions
- signed release artifacts
- reproducible-build investigation
- review of generated WebAssembly/mobile artifacts
- protected release credentials

The current CI adds formatting, tests, Clippy and dependency advisory checks. This is hardening, not a supply-chain proof.

## Claim boundary

A defensible statement is:

> Sigil is designed to reduce plaintext and metadata exposure against capable network and infrastructure adversaries while failing closed on unauthenticated content.

An indefensible statement is:

> Sigil prevents a state intelligence service from identifying, investigating or compromising its users.
