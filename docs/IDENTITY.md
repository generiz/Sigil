# Identity

Sigil separates what the human recognizes from what the protocol trusts.

## No real names by default

The secure contact UI does not require a real name, phone number or global username.

A contact may have a random local alias such as:

```text
A73F-19C2-6D04-8BE1
```

The alias is presentation only. It may rotate and is never proof of identity.

The normal conversation view can be even quieter: a verified contact may be represented primarily by a local visual marker such as a colored dot plus a verification state. Real-world names do not need to appear in the secure conversation surface.

## Cryptographic identity

Protocol trust is anchored to public identity material or a hardware-backed key handle that can expose verifiable public material.

Conceptually a contact contains:

```text
pinned public identity key
fingerprint
trust state
optional random alias
local visual derivation state
```

It does not need a legal or social name.

## Verification

Intended flow:

1. receive a one-time invitation/session request
2. display only pseudonymous local presentation
3. compare the full fingerprint or scan a QR representation over an independent channel
4. pin the verified public identity key locally
5. mark the contact verified
6. derive the local visual marker from verified key material

The QR/fingerprint proves continuity with the key that was checked. It does not independently prove legal identity.

## Visual continuity

Sigil needs a small amount of continuity for the human without putting stable identity into the network.

A user may consistently recognize a verified peer as a local marker such as a green dot, shape or pattern. That marker is local presentation state. It is never transmitted as a peer identifier and cannot authenticate a contact by itself.

The intended invariant is:

```text
stable local recognition
!=
stable network identifier
```

Transport/mailbox/routing tokens can rotate per delivery epoch while the local UI continues to resolve authenticated messages back to the same verified identity.

## Key changes

A verified key change is a security event:

```text
Verified -> KeyChanged
```

The old trust state cannot silently carry forward. Re-verification is required before the replacement identity becomes trusted.

A matching alias or visual marker must never suppress a key-change warning.

## Invitations

Contact discovery should not require uploading an address book.

Preferred direction:

```text
one-time rendezvous token
+ public identity material
+ protocol version/capabilities
```

The invitation may be shared as a QR code or short-lived link. It is not a permanent username and should expire or become unusable after acceptance.

## Independent rotations

These states have different meanings and lifetimes:

- alias: local presentation
- visual render epoch: short-lived renderer state
- delivery epoch: short-lived routing/mailbox state
- message/session keys: cryptographic confidentiality state
- identity key: authentication/continuity anchor

They must not be collapsed into one identifier.

## Multi-device direction

A later design should distinguish root account identity from device identities. Adding a device requires an authenticated authorization event visible to existing verified devices.
