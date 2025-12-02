use enc::test;
use proto_packet_test::structs::fields::Primitives;

#[test]
fn primitives() {
    let primitives: Primitives = Primitives::new(1, 2, 3, 4, 5);
    let expected: &[u8] = &[1, 2, 3, 4, 5];
    test::test_io(&primitives, &expected, true);
}
