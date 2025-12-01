use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `i16`

    /// Decodes an `i16` value from the `Read` prefix with the `first` byte.
    pub fn decode_i16<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i16, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_i8(wire, r, first)? as i16,
            Fixed2Byte => i16::from_le_bytes(WireType::decode_fixed_2_byte(r, first)?),
            Fixed4Byte => {
                let value: i32 = self.decode_i32(wire, r, first)?;
                if value > i16::MAX as i32 || value < i16::MIN as i32 {
                    return Err(ValueOutOfRange);
                }
                value as i16
            }
            Fixed8Byte => {
                let value: i64 = self.decode_i64(wire, r, first)?;
                if value > i16::MAX as i64 || value < i16::MIN as i64 {
                    return Err(ValueOutOfRange);
                }
                value as i16
            }
            Fixed16Byte => {
                let value: i128 = self.decode_i128(wire, r, first)?;
                if value > i16::MAX as i128 || value < i16::MIN as i128 {
                    return Err(ValueOutOfRange);
                }
                value as i16
            }
            VarInt => {
                let value: i32 = self.decode_i32(wire, r, first)?;
                if value > i16::MAX as i32 || value < i16::MIN as i32 {
                    return Err(ValueOutOfRange);
                }
                value as i16
            }
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
