use std::error::Error;

use proto_packet_test::message_fields::{NamedTypes, SpecialTypes, UnsignedInts};

mod common;

#[test]
pub fn unsigned_ints() -> Result<(), Box<dyn Error>> {
    let message: UnsignedInts = UnsignedInts::default()
        .with_one(1)
        .with_two(12)
        .with_three(123)
        .with_four(1234)
        .with_five(12345);
    common::test_packet(
        &message,
        &[
            1, 1, 0xA2, 12, 0xA3, 123, 0xA4, 0xD2, 0x09, 0xA5, 0xB9, 0x60,
        ],
    )
}

#[test]
pub fn special_types() -> Result<(), Box<dyn Error>> {
    let message: SpecialTypes = SpecialTypes::default()
        .with_one(uuid::Uuid::from_bytes([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ]))
        .with_two("Hello, World!".to_string());
    common::test_packet(
        &message,
        &[
            0x81, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0xC2, 13, b'H', b'e', b'l',
            b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!',
        ],
    )
}

#[test]
fn named_types() -> Result<(), Box<dyn Error>> {
    let message: NamedTypes = NamedTypes::default().with_local_message(
        UnsignedInts::default()
            .with_one(1)
            .with_two(12)
            .with_three(123)
            .with_four(1234)
            .with_five(12345),
    );
    common::test_packet(
        &message,
        &[
            0xC1, 12, 1, 1, 0xA2, 12, 0xA3, 123, 0xA4, 0xD2, 0x09, 0xA5, 0xB9, 0x60,
        ],
    )
}
