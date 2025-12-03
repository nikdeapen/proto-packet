use enc::test;
use proto_packet_test::messages::fields::{Named, Primitives, Specials};
use uuid::Uuid;

#[test]
fn primitives() {
    let primitives: Primitives = Primitives::default()
        .with_one(1)
        .with_two(2)
        .with_three(3)
        .with_four(4)
        .with_five(5);
    let expected: &[u8] = &[0x01, 1, 0xA2, 2, 0xA3, 3, 0xA4, 4, 0xA5, 5];
    test::test_io(&primitives, &expected, true);
}

#[test]
fn specials() {
    let specials: Specials = Specials::default()
        .with_one(Uuid::from_bytes([
            1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8,
        ]))
        .with_two("Hello, World!".to_string());
    let expected: &[u8] = &[
        0x81, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 0xC2, 13, b'H', b'e', b'l', b'l',
        b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!',
    ];
    test::test_io(&specials, expected, true);
}

#[test]
fn named() {
    let primitives: Primitives = Primitives::default()
        .with_one(1)
        .with_two(2)
        .with_three(3)
        .with_four(4)
        .with_five(5);
    let named: Named = Named::default().with_one(primitives);
    let expected: &[u8] = &[0xC1, 10, 0x01, 1, 0xA2, 2, 0xA3, 3, 0xA4, 4, 0xA5, 5];
    test::test_io(&named, &expected, true);
}
