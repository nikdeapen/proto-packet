use enc::test::test_io;
use proto_packet_test::fields::enums::SingleCase;

#[test]
fn enums() {
    let packet: SingleCase = SingleCase::One;
    let encoded: &[u8] = &[1];
    test_io(&packet, encoded, false);
}
