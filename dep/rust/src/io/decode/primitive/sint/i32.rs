use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::var_int::VarInt32;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `i32`

    /// Decodes an `i32` value from the `Read` prefix with the `first` byte.
    pub fn decode_i32<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i32, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_i8(wire, r, first)? as i32,
            Fixed2Byte => self.decode_i16(wire, r, first)? as i32,
            Fixed4Byte => i32::from_le_bytes(WireType::decode_fixed_4_byte(r, first)?),
            Fixed8Byte => {
                let value: i64 = self.decode_i64(wire, r, first)?;
                if value > i32::MAX as i64 || value < i32::MIN as i64 {
                    return Err(ValueOutOfRange);
                }
                value as i32
            }
            Fixed16Byte => {
                let value: i128 = self.decode_i128(wire, r, first)?;
                if value > i32::MAX as i128 || value < i32::MIN as i128 {
                    return Err(ValueOutOfRange);
                }
                value as i32
            }
            VarInt => VarInt32::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(|e| DecodingError::from_var_int_error(e))?
                .to_zig_zag(),
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
