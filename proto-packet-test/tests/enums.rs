use std::error::Error;

use proto_packet_test::enums::{MultipleCases, SingleCase};

use crate::common::test_packet;

mod common;

#[test]
fn single_case() -> Result<(), Box<dyn Error>> {
    let enom: SingleCase = SingleCase::One;
    test_packet(&enom, &[1])
}

#[test]
fn multiple_cases() -> Result<(), Box<dyn Error>> {
    let enom: MultipleCases = MultipleCases::Two;
    test_packet(&enom, &[2])
}
