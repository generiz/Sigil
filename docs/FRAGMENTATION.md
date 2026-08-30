# Encrypted fragment layer

Sigil 0.3 adds a resilience layer that operates only on the already authenticated outer wire envelope.

It is not a replacement for end-to-end encryption and it never fragments semantic plaintext as a confidentiality mechanism.

## Required ordering

```text
SymbolId stream
  -> message-scoped symbol representation
  -> inner XChaCha20-Poly1305
  -> outer XChaCha20-Poly1305
  -> optional traffic-size padding
  -> erasure coding
  -> opaque fragments
  -> node distribution
```

The receiver reverses this order. It reconstructs the outer ciphertext first and only then attempts authenticated decryption.

## Default coding policy

The current default is:

```text
12 data shards
 8 parity shards
----------------
20 total shards
12 required to reconstruct
```

Any valid set of 12 of the 20 fragments is sufficient to reconstruct the encrypted wire object. Up to 8 missing fragments can therefore be tolerated.

The default intentionally spends bandwidth and storage to gain resilience. Applications can create a different validated `FragmentPolicy` when a different trade-off is required.

The current Galois-field implementation limits one coded set to 255 total shards. This is independent from the network node pool, which can contain up to 1000 nodes.

## Opaque fragments

A network-facing `OpaqueFragment` contains only:

```text
random 256-bit capability
opaque shard bytes
```

It does not contain its Reed-Solomon index, the original ciphertext length, a human recipient name, a contact alias or a permanent message identifier.

Fragments are shuffled before being returned for distribution.

The random capability is a short-lived locator, not an identity and not a cryptographic substitute for AEAD authentication.

## Endpoint manifest

The reconstruction information lives in `FragmentManifest` at the endpoint.

The manifest contains the information needed to map opaque capabilities back to coding positions, including:

- data/parity counts
- shard length
- original outer-ciphertext length
- capability-to-shard-index mapping
- BLAKE3 digest of the original outer ciphertext

This manifest is linkability-sensitive because it reveals which otherwise independent pieces belong together. Live protocol work must keep it endpoint-controlled or derive/protect it through authenticated session state.

Store nodes do not need the manifest.

## Random alignment padding

Before coding, the encrypted wire object is extended to an exact multiple of the data-shard count. Alignment bytes are generated randomly rather than filled with a fixed pattern.

This alignment padding exists for coding. It is not the traffic-size padding policy used to hide coarse message lengths.

If size-class padding is enabled, it should be applied to the encrypted transport object before fragment coding.

## Integrity model

Reed-Solomon coding repairs missing pieces. It is not a Byzantine-consensus system and does not make maliciously modified shards trustworthy.

Sigil currently uses two checks at different layers:

1. the endpoint manifest checks the BLAKE3 digest after reconstruction;
2. the reconstructed outer wire envelope must still pass XChaCha20-Poly1305 authentication before the inner layer is exposed.

The AEAD check is the cryptographic authority. The manifest digest is an early reconstruction-consistency check.

A future live transport may add authenticated per-fragment metadata so corrupted pieces can be discarded before attempting full reconstruction.

## Node distribution

`NodePool::targets_for_fragments()` spreads fragment destinations across the available pool.

When there are at least as many nodes as fragments, the current model selects a different node for every fragment in that distribution round.

When the pool is smaller than the fragment count, nodes are reused only after a shuffled pass through the pool, keeping placement approximately balanced.

Examples:

```text
100 nodes + 20 fragments -> 20 distinct target nodes
2 nodes + 20 fragments   -> 10 fragments per node
```

This selects storage/delivery targets only. A live client still needs independent route construction, retries, expiry, retrieval and authenticated protocol state.

## What this provides

Implemented and tested:

- fragmentation happens after both AEAD layers
- 12-of-20 default reconstruction
- recovery with eight pieces missing
- fragment ordering is unnecessary
- network-facing pieces omit their coding index
- random independent capability per piece
- reconstruction digest validation
- distribution across node pools from 2 to 1000 nodes
- end-to-end integration test from `SymbolId` through double AEAD, fragment loss, reconstruction and back to `SymbolId`

## What this does not provide

Not implemented yet:

- live storage nodes
- fragment upload/retrieval protocol
- fragment TTL/expiry enforcement
- authenticated manifest synchronization between devices
- ratchet-derived fragment capabilities
- per-fragment cryptographic authentication
- mix scheduling or traffic-correlation resistance
- protection against a global observer

Fragment dispersal increases resilience and can reduce how much encrypted state one storage node receives. It does not make traffic impossible to trace and does not add confidentiality beyond the cryptographic layers that precede it.
