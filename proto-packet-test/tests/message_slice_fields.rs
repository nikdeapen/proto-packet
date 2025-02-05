use std::error::Error;

use proto_packet_test::message_slice_fields::UnsignedInts;

mod common;

#[test]
pub fn unsigned_ints() -> Result<(), Box<dyn Error>> {
    let one: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let message: UnsignedInts = UnsignedInts::default().with_one(one);
    common::test_packet(&message, &[0xE1, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}
