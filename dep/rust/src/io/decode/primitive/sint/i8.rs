use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `i8`

    /// Decodes an `i8` value from the `Read` prefix with the `first` byte.
    pub fn decode_i8<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i8, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => first as i8,
            Fixed2Byte => {
                let value: i16 = self.decode_i16(wire, r, first)?;
                if value > i8::MAX as i16 || value < i8::MIN as i16 {
                    return Err(ValueOutOfRange);
                }
                value as i8
            }
            Fixed4Byte => {
                let value: i32 = self.decode_i32(wire, r, first)?;
                if value > i8::MAX as i32 || value < i8::MIN as i32 {
                    return Err(ValueOutOfRange);
                }
                value as i8
            }
            Fixed8Byte => {
                let value: i64 = self.decode_i64(wire, r, first)?;
                if value > i8::MAX as i64 || value < i8::MIN as i64 {
                    return Err(ValueOutOfRange);
                }
                value as i8
            }
            Fixed16Byte => {
                let value: i128 = self.decode_i128(wire, r, first)?;
                if value > i8::MAX as i128 || value < i8::MIN as i128 {
                    return Err(ValueOutOfRange);
                }
                value as i8
            }
            VarInt => {
                let value: i32 = self.decode_i32(wire, r, first)?;
                if value > i8::MAX as i32 || value < i8::MIN as i32 {
                    return Err(ValueOutOfRange);
                }
                value as i8
            }
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
