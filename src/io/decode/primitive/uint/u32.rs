use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt32;
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
            Fixed4Byte => u32::from_le_bytes(WireType::decode_fixed_4_byte(r, first)?),
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
                .map_err(DecodingError::from_var_int_error)?
                .value(),
            _ => return Err(InvalidWireType(wire)),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_fixed_1_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u32 = decoder.decode_u32(Fixed1Byte, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_2_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u32 = decoder
            .decode_u32(Fixed2Byte, &mut &[0x03u8][..], 0x02)
            .unwrap();
        assert_eq!(result, 0x0302);
    }

    #[test]
    fn decode_fixed_4_byte() {
        let decoder: Decoder = Decoder::default();
        // 0x04030201 le = [0x01, 0x02, 0x03, 0x04]
        let result: u32 = decoder
            .decode_u32(Fixed4Byte, &mut &[0x02u8, 0x03, 0x04][..], 0x01)
            .unwrap();
        assert_eq!(result, 0x04030201);
    }

    #[test]
    fn decode_fixed_8_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u32 = decoder
            .decode_u32(Fixed8Byte, &mut &[0u8, 0, 0, 0, 0, 0, 0][..], 42)
            .unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_8_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        // 0x0000000100000000 > u32::MAX
        let result: Result<u32, DecodingError> =
            decoder.decode_u32(Fixed8Byte, &mut &[0u8, 0, 0, 1, 0, 0, 0][..], 0);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_varint() {
        let decoder: Decoder = Decoder::default();
        let result: u32 = decoder.decode_u32(VarInt, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_varint_multi_byte() {
        let decoder: Decoder = Decoder::default();
        // varint 300 = [0xAC, 0x02]
        let result: u32 = decoder
            .decode_u32(VarInt, &mut &[0x02u8][..], 0xAC)
            .unwrap();
        assert_eq!(result, 300);
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<u32, DecodingError> =
            decoder.decode_u32(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(result, Err(DecodingError::InvalidWireType(LengthPrefixed))));
    }
}
