use enc::test;
use proto_packet_test::structs::lists::{Named, Primitives, Specials};
use uuid::Uuid;

#[test]
fn primitives() {
    let primitives: Primitives = Primitives::new(
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
        vec![13, 14, 15],
    );
    let expected: &[u8] = &[
        0x03,
        1,
        2,
        3,
        0b1010_0011,
        4,
        5,
        6,
        0b1010_0011,
        7,
        8,
        9,
        0b1010_0011,
        10,
        11,
        12,
        0b1010_0011,
        13,
        14,
        15,
    ];
    test::test_io(&primitives, expected, true);
}

#[test]
fn specials() {
    let one: Vec<Uuid> = vec![
        Uuid::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
        Uuid::from_bytes([0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]),
    ];
    let two: Vec<String> = vec!["A".to_string(), "B".to_string()];
    let specials: Specials = Specials::new(one, two);
    let expected: &[u8] = &[
        0b1001_1111,
        2,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        0b1100_0100,
        1,
        b'A',
        1,
        b'B',
    ];
    test::test_io(&specials, expected, true);
}

#[test]
fn named() {
    let primitives: Primitives = Primitives::new(
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
        vec![13, 14, 15],
    );
    let named: Named = Named::new(vec![primitives.clone(), primitives]);
    let expected: &[u8] = &[
        0b1101_1111, // named list header
        12,          // named list header
        20,          // primitives list header
        3,
        1,
        2,
        3,
        0b1010_0011,
        4,
        5,
        6,
        0b1010_0011,
        7,
        8,
        9,
        0b1010_0011,
        10,
        11,
        12,
        0b1010_0011,
        13,
        14,
        15,
        20,
        0x03,
        1,
        2,
        3,
        0b1010_0011,
        4,
        5,
        6,
        0b1010_0011,
        7,
        8,
        9,
        0b1010_0011,
        10,
        11,
        12,
        0b1010_0011,
        13,
        14,
        15,
    ];
    test::test_io(&named, expected, true);
}
