use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::var_int::VarInt64;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `i64`

    /// Decodes an `i64` value from the `Read` prefix with the `first` byte.
    pub fn decode_i64<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i64, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_i8(wire, r, first)? as i64,
            Fixed2Byte => self.decode_i16(wire, r, first)? as i64,
            Fixed4Byte => self.decode_i32(wire, r, first)? as i64,
            Fixed8Byte => i64::from_le_bytes(WireType::decode_fixed_8_byte(r, first)?),
            Fixed16Byte => {
                let value: i128 = self.decode_i128(wire, r, first)?;
                if value > i64::MAX as i128 || value < i64::MIN as i128 {
                    return Err(ValueOutOfRange);
                }
                value as i64
            }
            VarInt => VarInt64::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_var_int_error)?
                .to_zig_zag(),
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
