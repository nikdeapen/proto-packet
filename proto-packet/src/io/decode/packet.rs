use std::io::{Error, Read};

use crate::io::decode::error::error_invalid_wire_type;
use crate::io::WireType;
use crate::Packet;

/// Decodes a `Packet` value from the `Read`.
pub fn decode_packet<R, P>(wire_type: WireType, r: &mut R) -> Result<P, Error>
where
    R: Read,
    P: Packet,
{
    if wire_type == P::wire_type() {
        P::decode_from_read_prefix(r)
    } else {
        Err(error_invalid_wire_type("Packet", wire_type))
    }
}
