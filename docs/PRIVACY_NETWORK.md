# Privacy network

Sigil is an online messenger. The client should use a dedicated transport stack rather than an embedded browser.

## Goal

Reduce metadata concentration so that no single Sigil-operated component is designed to know all of:

- the client's original network address
- the peer's cryptographic identity
- the destination mailbox meaning
- the plaintext message content

End-to-end encryption protects content. Relay separation limits what individual infrastructure components can learn.

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

The receiver uses an independent route to fetch from the mailbox layer.

## Relay roles

### Entry relay

May observe the source network connection.

It should not receive a human contact name, peer identity label or plaintext message. It forwards opaque protocol traffic toward the next hop.

### Transit relay

Forwards opaque traffic between route layers.

It should not receive application plaintext or a stable human identity. Its purpose is to prevent the entry and mailbox roles from being collapsed into one observer.

### Mailbox relay

Stores and delivers opaque encrypted envelopes using a mailbox token.

It should not need the sender's original network address or a human-readable recipient identity.

## Mailbox tokens

A mailbox token is an opaque capability used for delivery. It is not a username.

Tokens should be scoped, replaceable and derived or distributed through authenticated session state. A server should not be able to convert a mailbox token into a real-world name from Sigil's protocol database.

## VPN use

A VPN or encrypted tunnel can protect the local network path and hide Sigil traffic from the immediate access network.

It does not create guaranteed anonymity. The VPN endpoint can observe the client's connection, and traffic timing/volume can still be correlated by sufficiently capable observers.

For that reason, VPN/tunnel protection is considered an outer transport layer, not the identity or anonymity mechanism.

## Push notifications

Push providers should receive no message text, sender alias or media preview.

A push notification should mean only that opaque data may be available. The client then retrieves encrypted envelopes through the normal privacy route.

## Traffic analysis

Multi-hop routing does not defeat a global passive observer by itself.

Potential future mitigations can include batching, bounded delivery delay, route rotation and size classes, but these must be evaluated against battery, latency and bandwidth costs. Sigil should not introduce large amounts of deceptive cover traffic merely to support an unprovable anonymity claim.

## Browser boundary

A general-purpose browser or WebView is not part of the trusted message-delivery path.

Links initially open externally with explicit user action. A future isolated viewer must run outside the secure composition and cryptographic state domains and must not receive conversation keys or message buffers.
