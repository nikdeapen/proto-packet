use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `u8`

    /// Decodes the `u8` value from the `Read` prefix with the `first` byte.
    pub fn decode_u8<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<u8, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => first,
            Fixed2Byte => {
                let value: u16 = self.decode_u16(wire, r, first)?;
                if value > u8::MAX as u16 {
                    return Err(ValueOutOfRange);
                }
                value as u8
            }
            Fixed4Byte => {
                let value: u32 = self.decode_u32(wire, r, first)?;
                if value > u8::MAX as u32 {
                    return Err(ValueOutOfRange);
                }
                value as u8
            }
            Fixed8Byte => {
                let value: u64 = self.decode_u64(wire, r, first)?;
                if value > u8::MAX as u64 {
                    return Err(ValueOutOfRange);
                }
                value as u8
            }
            Fixed16Byte => {
                let value: u128 = self.decode_u128(wire, r, first)?;
                if value > u8::MAX as u128 {
                    return Err(ValueOutOfRange);
                }
                value as u8
            }
            VarInt => {
                let value: u32 = self.decode_u32(wire, r, first)?;
                if value > u8::MAX as u32 {
                    return Err(ValueOutOfRange);
                }
                value as u8
            }
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
