# Identity

Sigil separates what the user sees from what the protocol trusts.

## No real names by default

The secure contact UI does not require a real name, phone number or global username.

A contact is represented by a random local alias such as:

```text
A73F-19C2-6D04-8BE1
```

The alias exists only for presentation. It may rotate and must never be treated as proof of identity.

## Cryptographic identity

The protocol identity is a public key or a hardware-backed key handle that can produce verifiable public identity material.

A contact record conceptually contains:

```text
visible alias
pinned public identity key
fingerprint
trust state
```

It does not need to contain a real-world name.

## Verification

The intended verification flow is out-of-band:

1. receive a contact invitation or session request
2. display the contact using a random alias
3. compare the full identity fingerprint or scan a QR code through an independent channel
4. pin the verified public identity key locally
5. mark the contact as verified

The QR/fingerprint proves continuity with the key that was checked. It does not independently prove a person's legal identity.

## Key changes

A verified key change is a security event.

Sigil should transition the contact to:

```text
Verified -> KeyChanged
```

The conversation must not silently keep the old verified state. The user must explicitly verify the replacement key before trust is restored.

The visible alias may stay the same or rotate; neither choice changes the security meaning of the key change.

## Invitations

Contact discovery should not require uploading an address book.

The preferred direction is one-time invitation material:

```text
random rendezvous token
+ public identity material
+ protocol version/capabilities
```

The invitation can be shared as a QR code or short-lived link. The rendezvous token is not a permanent username and should expire or become unusable after acceptance.

## Alias rotation

Alias rotation is independent of session-key rotation and identity-key rotation.

This keeps three concepts separate:

- alias: presentation only
- session keys: message confidentiality state
- identity key: continuity/authentication anchor

A secure design must never infer trust from the alias.

## Multi-device direction

A later multi-device design should distinguish a root account identity from individual device identities. Adding a device should require an authenticated authorization event and should be visible to existing verified devices.
