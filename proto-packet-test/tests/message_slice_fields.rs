use std::error::Error;

use proto_packet_test::message_fields::UnsignedInts;
use proto_packet_test::message_slice_fields::{MessageNamedTypeSlices, MessageUnsignedIntSlices};

mod common;

#[test]
fn unsigned_ints() -> Result<(), Box<dyn Error>> {
    let one: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let message: MessageUnsignedIntSlices = MessageUnsignedIntSlices::default().with_one(one);
    common::test_packet(&message, &[0xE1, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}

#[test]
fn named_types() -> Result<(), Box<dyn Error>> {
    let packet: UnsignedInts = UnsignedInts::default().with_one(1);
    let one: Vec<UnsignedInts> = vec![packet.clone(), packet.clone(), packet.clone()];
    let message: MessageNamedTypeSlices = MessageNamedTypeSlices::default().with_one(one);
    common::test_packet(&message, &[0xE1, 0xC9, 2, 1, 1, 2, 1, 1, 2, 1, 1])
}
