use std::error::Error;

use uuid::Uuid;

use proto_packet_test::variant_fields::{NamedTypes, SpecialTypes, UnsignedInts};

mod common;

#[test]
pub fn unsigned_ints() -> Result<(), Box<dyn Error>> {
    let variant: UnsignedInts = UnsignedInts::Two(12345);
    common::test_packet(&variant, &[0xA2, 0xB9, 0x60])
}

#[test]
pub fn special_types() -> Result<(), Box<dyn Error>> {
    let variant: SpecialTypes = SpecialTypes::One(Uuid::from_bytes([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]));
    common::test_packet(
        &variant,
        &[0x81, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    )?;

    let variant: SpecialTypes = SpecialTypes::Two("Hello, World!".to_string());
    common::test_packet(
        &variant,
        &[
            0xC2, 13, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!',
        ],
    )?;

    Ok(())
}

#[test]
fn named_types() -> Result<(), Box<dyn Error>> {
    let variant: NamedTypes = NamedTypes::LocalVariant(UnsignedInts::Two(12345));
    common::test_packet(&variant, &[0xC1, 3, 0xA2, 0xB9, 0x60])
}
