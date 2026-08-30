# Privacy network

Sigil is an online messenger with a dedicated transport stack. A general-purpose browser is not part of the message path.

## Goal

Reduce metadata concentration and linkability so that no single Sigil-operated component is designed to know all of:

- the client's original network address
- the peer's cryptographic identity
- the destination mailbox meaning
- plaintext content

End-to-end encryption protects content. Relay separation and ephemeral delivery state reduce what individual infrastructure components can learn.

## Route model

```text
Client
  |
optional VPN / encrypted tunnel
  |
Entry relay
  |
Transit relay
  |
Mailbox relay
```

The receiver uses an independent route to retrieve opaque envelopes.

## Relay roles

### Entry

May observe the source network connection. It should not receive peer identity, a human contact label, plaintext or the final mailbox token.

### Transit

Forwards opaque traffic between route layers. It should not receive application plaintext or stable human identity.

### Mailbox

Stores/delivers opaque envelopes addressed by short-lived mailbox capabilities. It should not require the sender's original network address or a human-readable recipient identity.

## Ephemeral delivery state

A mailbox token is not a username. The protocol target is for delivery state to rotate aggressively.

Conceptually:

```text
message epoch N
  routing token A
  mailbox token B

message epoch N+1
  routing token C
  mailbox token D
```

The current Rust core can generate independent opaque delivery epochs. A future authenticated ratchet/session must derive or distribute these values so both endpoints can advance safely without exposing a permanent recipient identifier.

Repeated random tokens by themselves do not create anonymity. Correct synchronization, replay handling, relay behavior and cryptographic binding still need implementation and review.

## Traffic-size classes

Exact ciphertext length can leak information. Sigil models fixed size classes for small envelopes so different payload lengths can share the same external size.

Current model classes are 4 KiB, 16 KiB, 64 KiB and 256 KiB. Larger media uses a separate chunking strategy.

Padding consumes bandwidth and does not conceal timing or direction by itself.

## Privacy modes

The core models three policy targets:

- `Standard`: fresh delivery tokens, latency-focused
- `Private`: token rotation + size classes + route rotation target
- `Maximum`: adds batching and bounded-delay targets, accepting higher latency/bandwidth cost

`Maximum` is a design target, not a claim that a mix network is currently implemented.

## Timing correlation

Even perfect payload encryption does not hide the fact that packets exist. An observer watching both ends may correlate timing, direction, volume and bursts.

The maximum-privacy transport may therefore evaluate:

- route rotation
- batching messages into delivery windows
- bounded randomized delay
- size classes/padding

These reduce some easy correlations at a cost in latency and battery. They do not guarantee anonymity against a global passive observer.

Sigil does not claim that an IP address becomes impossible to trace.

## VPN use

A VPN/encrypted tunnel can protect the local path and move first-hop visibility away from the access network. The VPN endpoint can still observe the client's connection, so VPN protection is an outer layer, not the identity/anonymity mechanism.

## Push notifications

Push providers should receive no message text, sender alias, visual marker or media preview. A push should carry only an opaque wake-up signal; the client retrieves and authenticates data through the privacy route.

## Browser boundary

Links initially open externally with explicit user action. A future isolated link viewer must run outside the secure composition, visual and cryptographic state domains and must not receive conversation keys or sensitive buffers.
