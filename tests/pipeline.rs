use sigil_core::{
    FragmentBundle, FragmentPolicy, LayeredEnvelope, MessageSecret, NodeId, NodePool,
    SecureSymbolStream, SymbolId, SymbolMapKey, TransportSecret,
};
use std::collections::HashSet;

#[test]
fn symbol_message_roundtrips_without_os_text_representation() {
    let alphabet: Vec<_> = (1..=40).map(SymbolId).collect();
    let original = [SymbolId(8), SymbolId(15), SymbolId(12), SymbolId(1)];

    let message_secret = MessageSecret::random();
    let transport_secret = TransportSecret::random();
    let symbol_key = SymbolMapKey::from_message_secret(&message_secret);
    let outgoing = SecureSymbolStream::encode(&original, &symbol_key);

    let envelope = LayeredEnvelope::seal(
        outgoing.as_bytes(),
        &message_secret,
        &transport_secret,
        b"verified-session",
        b"ephemeral-route",
    )
    .unwrap();

    let wire = envelope.to_wire_bytes();
    let received = LayeredEnvelope::from_wire_bytes(&wire).unwrap();
    let decoded_bytes = received
        .open(
            &message_secret,
            &transport_secret,
            b"verified-session",
            b"ephemeral-route",
        )
        .unwrap();
    let incoming = SecureSymbolStream::from_encoded(decoded_bytes).unwrap();

    for (index, expected) in original.iter().enumerate() {
        assert_eq!(
            incoming
                .decode_symbol_at(index, &alphabet, &symbol_key)
                .unwrap(),
            *expected
        );
    }
}

#[test]
fn encrypted_message_survives_fragment_loss_before_decryption() {
    let alphabet: Vec<_> = (1..=40).map(SymbolId).collect();
    let original = [
        SymbolId(19),
        SymbolId(9),
        SymbolId(7),
        SymbolId(9),
        SymbolId(12),
    ];

    let message_secret = MessageSecret::random();
    let transport_secret = TransportSecret::random();
    let symbol_key = SymbolMapKey::from_message_secret(&message_secret);
    let outgoing = SecureSymbolStream::encode(&original, &symbol_key);

    let envelope = LayeredEnvelope::seal(
        outgoing.as_bytes(),
        &message_secret,
        &transport_secret,
        b"verified-session",
        b"ephemeral-route",
    )
    .unwrap();

    let outer_ciphertext = envelope.to_wire_bytes();
    let bundle = FragmentBundle::split(&outer_ciphertext, FragmentPolicy::default()).unwrap();

    let pool = NodePool::new((0..100).map(|_| NodeId::random()).collect()).unwrap();
    let targets = pool
        .targets_for_fragments(bundle.fragments().len())
        .unwrap();
    let unique_targets: HashSet<_> = targets.iter().copied().collect();
    assert_eq!(targets.len(), 20);
    assert_eq!(unique_targets.len(), 20);

    let available = bundle.fragments()[8..].to_vec();
    let reconstructed_outer = bundle.manifest().reconstruct(&available).unwrap();
    assert_eq!(reconstructed_outer, outer_ciphertext);

    let received = LayeredEnvelope::from_wire_bytes(&reconstructed_outer).unwrap();
    let decoded_bytes = received
        .open(
            &message_secret,
            &transport_secret,
            b"verified-session",
            b"ephemeral-route",
        )
        .unwrap();
    let incoming = SecureSymbolStream::from_encoded(decoded_bytes).unwrap();

    for (index, expected) in original.iter().enumerate() {
        assert_eq!(
            incoming
                .decode_symbol_at(index, &alphabet, &symbol_key)
                .unwrap(),
            *expected
        );
    }
}
