use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::var_int::VarInt32;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `u32`

    /// Decodes the `u32` value from the `Read` prefix with the `first` byte.
    pub fn decode_u32<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<u32, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_u8(wire, r, first)? as u32,
            Fixed2Byte => self.decode_u16(wire, r, first)? as u32,
            Fixed4Byte => {
                let mut value: [u8; 4] = [first, 0, 0, 0];
                r.read_exact(&mut value[1..]).map_err(|e| Stream(e))?;
                u32::from_le_bytes(value)
            }
            Fixed8Byte => {
                let value: u64 = self.decode_u64(wire, r, first)?;
                if value > u32::MAX as u64 {
                    return Err(ValueOutOfRange);
                }
                value as u32
            }
            Fixed16Byte => {
                let value: u128 = self.decode_u128(wire, r, first)?;
                if value > u32::MAX as u128 {
                    return Err(ValueOutOfRange);
                }
                value as u32
            }
            VarInt => VarInt32::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(|e| DecodingError::from_var_int_error(e))?
                .value(),
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
