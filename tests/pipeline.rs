use sigil_core::{
    LayeredEnvelope, MessageSecret, SecureSymbolStream, SymbolId, SymbolMapKey, TransportSecret,
};

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
