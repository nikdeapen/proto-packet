use enc::test;
use proto_packet_test::structs::fields::{Named, Primitives, Specials};
use uuid::Uuid;

#[test]
fn primitives() {
    let primitives: Primitives = Primitives::new(1, 2, 3, 4, 5, -1, -2, -3, -4, -5);
    let expected: &[u8] = &[1, 2, 3, 4, 5, 0xFF, 3, 5, 7, 9];
    test::test_io(&primitives, &expected, true);
}

#[test]
fn specials() {
    let specials: Specials = Specials::new(
        Uuid::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
        "Hello, World!".to_string(),
    );
    let expected: &[u8] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 13, b'H', b'e', b'l', b'l', b'o', b',',
        b' ', b'W', b'o', b'r', b'l', b'd', b'!',
    ];
    test::test_io(&specials, expected, true);
}

#[test]
fn named() {
    let primitives: Primitives = Primitives::new(1, 2, 3, 4, 5, -1, -2, -3, -4, -5);
    let named: Named = Named::new(primitives);
    let expected: &[u8] = &[10, 1, 2, 3, 4, 5, 0xFF, 3, 5, 7, 9];
    test::test_io(&named, &expected, true);
}
