use std::io;
use std::io::Read;

use enc::DecodeFromReadPrefix;

use crate::io::decode::error::error_invalid_wire_type;
use crate::io::WireType::{Fixed1Byte, List};
use crate::io::{ListHeader, WireType};

pub fn decode_slice_u8<R>(wire_type: WireType, r: &mut R) -> Result<Vec<u8>, io::Error>
where
    R: Read,
{
    if wire_type != List {
        return Err(error_invalid_wire_type("[]u8", wire_type));
    }

    let list_header: ListHeader = ListHeader::decode_from_read_prefix(r)?;
    if list_header.wire_type() != Fixed1Byte {
        return Err(error_invalid_wire_type("u8", list_header.wire_type()));
    }

    let mut result: Vec<u8> = vec![0u8; list_header.list_size_bytes()];
    r.read_exact(&mut result)?;
    Ok(result)
}
