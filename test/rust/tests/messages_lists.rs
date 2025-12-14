use enc::{test, EncodeToSlice};
use proto_packet::io::WireType::{Fixed16Byte, LengthPrefixed, List};
use proto_packet::io::{FieldHeader, ListHeader, TagNumber};
use proto_packet_test::messages::lists::{Named, Primitives, Specials};
use uuid::Uuid;

fn primitives_value() -> Primitives {
    Primitives::default()
        .with_one(vec![1, 2, 3])
        .with_two(vec![2, 3, 4])
        .with_three(vec![3, 4, 5])
        .with_four(vec![4, 5, 6])
        .with_five(vec![5, 6, 7])
}

fn primitives_encoded() -> Vec<u8> {
    let one: &[u8] = &[0xC1, 3, 1, 2, 3];
    let two: &[u8] = &[0xE2, 0b1010_0011, 2, 3, 4];
    let thr: &[u8] = &[0xE3, 0b1010_0011, 3, 4, 5];
    let fou: &[u8] = &[0xE4, 0b1010_0011, 4, 5, 6];
    let fiv: &[u8] = &[0xE5, 0b1010_0011, 5, 6, 7];
    [one, two, thr, fou, fiv].concat()
}

#[test]
fn primitives() {
    let primitives: Primitives = primitives_value();
    let expected: Vec<u8> = primitives_encoded();
    test::test_io(&primitives, &expected, true);
}

#[test]
fn specials() {
    let uuid: Uuid = Uuid::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
    let string: String = "Hello, World!".to_string();
    let specials: Specials = Specials::default()
        .with_one(vec![uuid, uuid])
        .with_two(vec![string.clone(), string.clone()]);

    let one: Vec<u8> = [
        FieldHeader::new(List, TagNumber::new(1).unwrap())
            .encode_as_vec()
            .unwrap()
            .as_slice(),
        ListHeader::new(Fixed16Byte, 32)
            .encode_as_vec()
            .unwrap()
            .as_slice(),
        uuid.as_bytes().as_slice(),
        uuid.as_bytes().as_slice(),
    ]
    .concat();
    let two: Vec<u8> = [
        FieldHeader::new(List, TagNumber::new(2).unwrap())
            .encode_as_vec()
            .unwrap()
            .as_slice(),
        ListHeader::new(LengthPrefixed, string.len() * 2 + 2)
            .encode_as_vec()
            .unwrap()
            .as_slice(),
        [13].as_slice(),
        string.as_bytes(),
        [13].as_slice(),
        string.as_bytes(),
    ]
    .concat();
    let expected: Vec<u8> = [one, two].concat();

    test::test_io(&specials, &expected, true);
}

#[test]
fn named() {
    let named: Named = Named::default().with_one(vec![primitives_value(), primitives_value()]);
    let primitives_len: usize = primitives_encoded().len();
    let expected: Vec<u8> = [
        FieldHeader::new(List, TagNumber::new(1).unwrap())
            .encode_as_vec()
            .unwrap(),
        ListHeader::new(LengthPrefixed, primitives_len * 2 + 2)
            .encode_as_vec()
            .unwrap(),
        vec![primitives_len as u8],
        primitives_encoded(),
        vec![primitives_len as u8],
        primitives_encoded(),
    ]
    .concat();
    test::test_io(&named, &expected, true);
}
