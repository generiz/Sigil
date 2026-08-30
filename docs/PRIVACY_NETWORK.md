# Privacy network

Sigil is an online messenger with a dedicated transport stack. A browser/WebView is not part of the message-delivery path.

## Goal

Reduce metadata concentration and stable identifiers without claiming impossible anonymity.

The network model is designed so no single node role needs all of:

- the client's original network address
- the peer's cryptographic identity
- a permanent recipient identifier
- plaintext content

End-to-end encryption protects content. Ephemeral delivery state, fragment capabilities and role separation limit what individual infrastructure components are designed to learn.

## No permanent mailbox

Sigil does not require a stable mailbox identifier in the core model.

Each delivery epoch creates fresh opaque state:

```text
MessageEpoch
DeliveryToken
RoutingToken
```

These are bounded protocol values, not usernames or human identities.

A future authenticated ratchet/session must derive or distribute them safely so endpoints advance without exposing a permanent network handle.

## Node pool

The Rust model supports a pool containing 2 to 1000 distinct nodes.

A route can select a distinct subset for one path. Fragment placement can additionally spread independent encrypted pieces across the pool.

For a 20-piece default fragment set:

```text
100-node pool -> 20 distinct target nodes
  2-node pool -> balanced shuffled reuse
```

The current code models target selection only. Live route establishment, upload, retrieval and server behavior are not implemented yet.

## Node knowledge

### Entry

May observe the incoming client network connection.

It is not designed to receive peer identity, plaintext or the final delivery token.

### Transit

Forwards opaque transport state.

It is not designed to require a human identity or application plaintext.

### Store/delivery

May receive a short-lived capability and opaque encrypted fragment.

It is not designed to require the original client network address, peer identity, fragment coding index or plaintext.

## Encrypted pieces

Sigil 0.3 implements the endpoint coding model for distributed pieces.

The ordering is fixed:

```text
inner AEAD
  -> outer AEAD
  -> optional traffic-size padding
  -> Reed-Solomon coding
  -> opaque fragments
  -> node placement
```

The default set contains 20 fragments: 12 data and 8 parity. Any 12 valid pieces reconstruct the encrypted outer wire object.

Each network-facing fragment carries a random 256-bit capability and opaque shard bytes. The capability-to-coding-index map remains in the endpoint manifest and is not needed by a store node.

Reed-Solomon improves loss tolerance. It does not add confidentiality and does not by itself prevent traffic correlation.

See `FRAGMENTATION.md`.

## Privacy policy

The core models three policy targets:

- `Standard`: fresh delivery state, latency-focused
- `Private`: adds size-class padding and route rotation targets
- `Maximum`: adds batching and bounded-delay targets, accepting higher latency and bandwidth cost

`Maximum` is the default policy target.

These are policy flags, not proof that a mix network exists.

## Size leakage

Ciphertext size can reveal information.

Sigil models size classes of 4 KiB, 16 KiB, 64 KiB and 256 KiB for small envelopes. Larger media uses its own chunking path.

The fragment layer also adds random alignment bytes so the encrypted wire object fits an exact number of data shards. Alignment padding is not a substitute for size-class padding.

## Timing correlation

An observer watching both sides of routes may correlate timing, direction, volume and bursts even when payload encryption is perfect.

Future transport experiments may evaluate:

- independent route selection per fragment or bounded fragment groups
- route rotation
- batching into delivery windows
- bounded randomized delay
- size classes/padding

These techniques trade latency, bandwidth and battery for reduced simple linkability. They do not guarantee anonymity against a sufficiently capable global observer.

Sigil does not claim that an IP address becomes impossible to trace.

## VPN use

A VPN or encrypted tunnel may protect the local first hop and move immediate network visibility away from the access provider.

The VPN endpoint can still observe the client's connection. VPN use therefore remains an outer transport option, not a proof of anonymity.

## Push notifications

Push providers should receive no message text, contact alias, visual marker or media preview.

A push should carry only an opaque wake-up signal; the client then retrieves and authenticates data through the normal transport path.

## Browser boundary

Links initially open externally with explicit user action.

Any future isolated link viewer must run outside the secure composition, cryptographic and visual state domains and must not receive message or transport secrets.
