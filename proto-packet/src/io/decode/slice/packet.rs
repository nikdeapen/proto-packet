use std::io;
use std::io::Read;

use enc::{read_optional_byte, DecodeFromReadPrefix, ExactRead};

use crate::io::decode::error::error_invalid_wire_type;
use crate::io::WireType::{LengthPrefixed, List};
use crate::io::{ListHeader, WireType};
use crate::Packet;

pub fn decode_slice_packet<P, R>(wire_type: WireType, r: &mut R) -> Result<Vec<P>, io::Error>
where
    P: Packet,
    R: Read,
{
    if wire_type != List {
        return Err(error_invalid_wire_type("[]Packet", wire_type));
    }

    let list_header: ListHeader = ListHeader::decode_from_read_prefix(r)?;
    if list_header.wire_type() != P::wire_type() {
        return Err(error_invalid_wire_type("packet", list_header.wire_type()));
    }

    let mut result: Vec<P> = Vec::default();
    let mut r: Box<dyn Read> = Box::new(ExactRead::new(r, list_header.list_size_bytes()));

    if P::wire_type() == LengthPrefixed {
        while let Some(first) = read_optional_byte(&mut r)? {
            let packet: P = P::decode_from_read_length_prefixed_with_first_byte(first, &mut r)?;
            result.push(packet)
        }
    } else {
        while let Some(first) = read_optional_byte(&mut r)? {
            let packet: P = P::decode_from_read_prefix_with_first_byte(first, &mut r)?;
            result.push(packet)
        }
    }

    Ok(result)
}
