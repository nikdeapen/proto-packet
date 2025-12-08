use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::var_int::VarInt128;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `i128`

    /// Decodes an `i128` value from the `Read` prefix with the `first` byte.
    pub fn decode_i128<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<i128, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_i8(wire, r, first)? as i128,
            Fixed2Byte => self.decode_i16(wire, r, first)? as i128,
            Fixed4Byte => self.decode_i32(wire, r, first)? as i128,
            Fixed8Byte => self.decode_i64(wire, r, first)? as i128,
            Fixed16Byte => i128::from_le_bytes(WireType::decode_fixed_16_byte(r, first)?),
            VarInt => VarInt128::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_var_int_error)?
                .to_zig_zag(),
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
