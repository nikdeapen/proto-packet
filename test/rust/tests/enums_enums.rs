use enc::test;
use proto_packet_test::enums::enums::{MultipleCases, SingleCase};

#[test]
#[ignore]
fn empty() {
    todo!("empty enums")
}

#[test]
fn single() {
    let single: SingleCase = SingleCase::One;
    test::test_io(&single, &[1], false);
}

#[test]
fn multiple() {
    let multiple: MultipleCases = MultipleCases::Three;
    test::test_io(&multiple, &[3], false);
}
