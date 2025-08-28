use enc::test::test_io;
use proto_packet_test::fields::messages::{NamedTypes, PrimitiveTypes, SpecialTypes};
use uuid::Uuid;

#[test]
fn primitive_types() {
    let packet: PrimitiveTypes = PrimitiveTypes::default()
        .with_one(1)
        .with_two(2)
        .with_three(3)
        .with_four(4)
        .with_five(5);
    let encoded: &[u8] = &[1, 1, 0xA2, 2, 0xA3, 3, 0xA4, 4, 0xA5, 5];
    test_io(&packet, encoded, true);
}

#[test]
fn special_types() {
    let packet: SpecialTypes = SpecialTypes::default()
        .with_one(Uuid::from_slice(&[0u8, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]).unwrap())
        .with_two("Hello, World!".to_string());
    let encoded: &[u8] = &[
        0x81, 0u8, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0xC2, 13, b'H', b'e', b'l', b'l',
        b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!',
    ];
    test_io(&packet, encoded, true);
}

#[test]
fn named_types() {
    let packet: NamedTypes = NamedTypes::default().with_one(PrimitiveTypes::default().with_one(1));
    let encoded: &[u8] = &[0xC1, 2, 1, 1];
    test_io(&packet, encoded, true);
}
