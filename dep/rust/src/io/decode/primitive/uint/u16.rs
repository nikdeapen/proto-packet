use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::var_int::VarInt16;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `u16`

    /// Decodes the `u16` value from the `Read` prefix with the `first` byte.
    pub fn decode_u16<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<u16, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_u8(wire, r, first)? as u16,
            Fixed2Byte => {
                let mut value: [u8; 2] = [first, 0];
                r.read_exact(&mut value[1..]).map_err(|e| Stream(e))?;
                u16::from_le_bytes(value)
            }
            Fixed4Byte => {
                let value: u32 = self.decode_u32(wire, r, first)?;
                if value > u16::MAX as u32 {
                    return Err(ValueOutOfRange);
                }
                value as u16
            }
            Fixed8Byte => {
                let value: u64 = self.decode_u64(wire, r, first)?;
                if value > u16::MAX as u64 {
                    return Err(ValueOutOfRange);
                }
                value as u16
            }
            Fixed16Byte => {
                let value: u128 = self.decode_u128(wire, r, first)?;
                if value > u16::MAX as u128 {
                    return Err(ValueOutOfRange);
                }
                value as u16
            }
            VarInt => VarInt16::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(|e| DecodingError::from_var_int_error(e))?
                .value(),
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
