use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read};

use uuid::Uuid;

use crate::io::decode::error::error_invalid_wire_type;
use crate::io::WireType;

/// Decodes a `uuid` value from the `Read`.
pub fn decode_uuid<R>(wire_type: WireType, r: &mut R) -> Result<Uuid, Error>
where
    R: Read,
{
    match wire_type {
        WireType::Fixed16Byte => {
            let value: [u8; 16] = WireType::decode_fixed_16_byte(r)?;
            Ok(Uuid::from_bytes(value))
        }
        _ => Err(error_invalid_wire_type("uuid", wire_type)),
    }
}

/// Decodes a `string` value from the `Read`.
pub fn decode_string<R>(wire_type: WireType, r: &mut R) -> Result<String, Error>
where
    R: Read,
{
    match wire_type {
        WireType::LengthPrefixed => {
            let value: Vec<u8> = WireType::decode_length_prefixed(r)?;
            String::from_utf8(value).map_err(|e| Error::new(InvalidData, e))
        }
        _ => Err(error_invalid_wire_type("uuid", wire_type)),
    }
}
