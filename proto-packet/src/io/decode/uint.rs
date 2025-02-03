use std::io::{Error, Read};

use enc::var_int::{VarInt128, VarInt16, VarInt32, VarInt64};
use enc::{read_single_byte, DecodeFromReadPrefix};

use crate::io::decode::error::error_invalid_wire_type;
use crate::io::WireType;
use crate::io::WireType::*;

/// Decodes a `u8` value from the `Read`.
pub fn decode_u8<R>(wire_type: WireType, r: &mut R) -> Result<u8, Error>
where
    R: Read,
{
    match wire_type {
        Fixed1Byte => read_single_byte(r),
        _ => Err(error_invalid_wire_type("u8", wire_type)),
    }
}

/// Decodes a `u16` value from the `Read`.
pub fn decode_u16<R>(wire_type: WireType, r: &mut R) -> Result<u16, Error>
where
    R: Read,
{
    match wire_type {
        Fixed8Byte => {
            let value: [u8; 2] = WireType::decode_fixed_2_byte(r)?;
            Ok(u16::from_le_bytes(value))
        }
        VarInt => Ok(VarInt16::decode_from_read_prefix(r)?.value),
        _ => Err(error_invalid_wire_type("u16", wire_type)),
    }
}

/// Decodes a `u32` value from the `Read`.
pub fn decode_u32<R>(wire_type: WireType, r: &mut R) -> Result<u32, Error>
where
    R: Read,
{
    match wire_type {
        Fixed8Byte => {
            let value: [u8; 4] = WireType::decode_fixed_4_byte(r)?;
            Ok(u32::from_le_bytes(value))
        }
        VarInt => Ok(VarInt32::decode_from_read_prefix(r)?.value),
        _ => Err(error_invalid_wire_type("u32", wire_type)),
    }
}

/// Decodes a `u64` value from the `Read`.
pub fn decode_u64<R>(wire_type: WireType, r: &mut R) -> Result<u64, Error>
where
    R: Read,
{
    match wire_type {
        Fixed8Byte => {
            let value: [u8; 8] = WireType::decode_fixed_8_byte(r)?;
            Ok(u64::from_le_bytes(value))
        }
        VarInt => Ok(VarInt64::decode_from_read_prefix(r)?.value),
        _ => Err(error_invalid_wire_type("u64", wire_type)),
    }
}

/// Decodes a `u128` value from the `Read`.
pub fn decode_u128<R>(wire_type: WireType, r: &mut R) -> Result<u128, Error>
where
    R: Read,
{
    match wire_type {
        Fixed16Byte => {
            let value: [u8; 16] = WireType::decode_fixed_16_byte(r)?;
            Ok(u128::from_le_bytes(value))
        }
        VarInt => Ok(VarInt128::decode_from_read_prefix(r)?.value),
        _ => Err(error_invalid_wire_type("u128", wire_type)),
    }
}
