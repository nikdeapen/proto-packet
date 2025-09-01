use chrono::NaiveDate;
use enc::test::test_io;
use proto_packet_test::fields::structs::{NamedTypes, SpecialTypes, UnsignedInts};
use uuid::Uuid;

#[test]
fn primitive_types() {
    let packet: UnsignedInts = UnsignedInts::new(1, 2u16, 3u32, 4u32, 5u64);
    let encoded: &[u8] = &[1, 2, 3, 4, 5];
    test_io(&packet, encoded, true);
}

#[test]
fn special_types() {
    let packet: SpecialTypes = SpecialTypes::new(
        Uuid::from_slice(&[0u8, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]).unwrap(),
        "Hello, World!".to_string(),
        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
    );
    let encoded: &[u8] = &[
        0u8, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 13, b'H', b'e', b'l', b'l', b'o', b',',
        b' ', b'W', b'o', b'r', b'l', b'd', b'!', 0,
    ];
    test_io(&packet, encoded, true);
}

#[test]
fn named_types() {
    let packet: NamedTypes = NamedTypes::new(UnsignedInts::new(1, 2u16, 3u32, 4u32, 5u64));
    let encoded: &[u8] = &[5, 1, 2, 3, 4, 5];
    test_io(&packet, encoded, true);
}
