# Privacy network

Sigil is an online messenger with a dedicated transport stack. A browser/WebView is not part of the message-delivery path.

## Goal

Reduce metadata concentration and stable identifiers without claiming impossible anonymity.

The network model is designed so no single node role needs all of:

- the client's original network address
- the peer's cryptographic identity
- a permanent recipient identifier
- plaintext content

End-to-end encryption protects content. Ephemeral delivery state and role separation limit what individual infrastructure components are designed to learn.

## No permanent mailbox

Sigil does not require a stable mailbox identifier in the core model.

Each delivery epoch creates fresh opaque state:

```text
MessageEpoch
DeliveryToken
RoutingToken
```

A `DeliveryToken` is not a username, account ID or human identity. It is bounded delivery state intended to rotate.

A future authenticated ratchet/session must derive or distribute these values safely so both endpoints advance without exposing a permanent network handle.

## Node pool

The current Rust model supports a pool containing 2 to 1000 distinct nodes.

An individual route selects only the nodes needed for that delivery:

```text
client
  |
entry
  |
zero or more transit nodes
  |
store/delivery
```

Routes contain distinct node IDs and can rotate independently between deliveries.

This is a model and policy layer. There are no live Sigil nodes or deployed routing protocol in the repository yet.

## Node knowledge

### Entry

May observe the incoming client network connection.

It is not designed to receive peer identity, plaintext or the final delivery token.

### Transit

Forwards opaque transport state.

It is not designed to require a human identity or application plaintext.

### Store/delivery

May receive a short-lived opaque delivery token and encrypted transport object.

It is not designed to require the original client network address, peer identity or plaintext.

## Privacy policy

The core models three policy targets:

- `Standard`: fresh delivery state, latency-focused
- `Private`: adds size-class padding and route rotation targets
- `Maximum`: adds batching and bounded-delay targets, accepting higher latency and bandwidth cost

`Maximum` is the default policy target in the current core.

These are policy flags, not proof that a mix network exists.

## Size leakage

Ciphertext size can reveal information.

Sigil currently models size classes of 4 KiB, 16 KiB, 64 KiB and 256 KiB for small envelopes. Larger media uses its own chunking path.

Padding hides exact length within a class but does not hide timing, direction or the fact that traffic exists.

## Timing correlation

An observer watching both sides of a route may correlate timing, direction, volume and bursts even when payload encryption is perfect.

Future transport experiments may evaluate:

- route rotation
- batching into delivery windows
- bounded randomized delay
- size classes/padding

These techniques trade latency, bandwidth and battery for reduced simple linkability. They do not guarantee anonymity against a sufficiently capable global observer.

Sigil does not claim that an IP address becomes impossible to trace.

## Future encrypted-piece delivery

A later phase may encode an already encrypted wire envelope into redundant pieces for resilient distributed storage/delivery.

The required order is:

```text
inner authenticated encryption
  -> outer authenticated encryption
  -> optional padding
  -> redundancy/fragmentation
  -> node delivery
```

Fragmentation must never be treated as encryption.

A piece should not require a human recipient name or permanent message identifier merely to be stored. Reconstruction state belongs at the authenticated endpoints or must be derived from authenticated session state.

This phase is not implemented yet.

## VPN use

A VPN or encrypted tunnel may protect the local first hop and move immediate network visibility away from the access provider.

The VPN endpoint can still observe the client's connection. VPN use therefore remains an outer transport option, not a proof of anonymity.

## Push notifications

Push providers should receive no message text, contact alias, visual marker or media preview.

A push should carry only an opaque wake-up signal; the client then retrieves and authenticates data through the normal transport path.

## Browser boundary

Links initially open externally with explicit user action.

Any future isolated link viewer must run outside the secure composition, cryptographic and visual state domains and must not receive message or transport secrets.
