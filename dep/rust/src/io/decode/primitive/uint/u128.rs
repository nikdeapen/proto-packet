use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::var_int::VarInt128;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `u128`

    /// Decodes the `u128` value from the `Read` prefix with the `first` byte.
    pub fn decode_u128<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<u128, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_u8(wire, r, first)? as u128,
            Fixed2Byte => self.decode_u16(wire, r, first)? as u128,
            Fixed4Byte => self.decode_u32(wire, r, first)? as u128,
            Fixed8Byte => self.decode_u64(wire, r, first)? as u128,
            Fixed16Byte => {
                let mut value: [u8; 16] = [0u8; 16];
                value[0] = first;
                r.read_exact(&mut value[1..]).map_err(|e| Stream(e))?;
                u128::from_le_bytes(value)
            }
            VarInt => VarInt128::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(|e| DecodingError::from_var_int_error(e))?
                .value(),
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
