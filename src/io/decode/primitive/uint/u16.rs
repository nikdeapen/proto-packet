use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt16;
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
            Fixed2Byte => u16::from_le_bytes(WireType::decode_fixed_2_byte(r, first)?),
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
        let result: u16 = decoder.decode_u16(Fixed1Byte, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_2_byte() {
        let decoder: Decoder = Decoder::default();
        // 0x0302 le = [0x02, 0x03]
        let result: u16 = decoder
            .decode_u16(Fixed2Byte, &mut &[0x03u8][..], 0x02)
            .unwrap();
        assert_eq!(result, 0x0302);
    }

    #[test]
    fn decode_fixed_4_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u16 = decoder
            .decode_u16(Fixed4Byte, &mut &[0u8, 0, 0][..], 42)
            .unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_4_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        // 0x00010000 = 65536 > u16::MAX, le = [0x00, 0x00, 0x01, 0x00]
        let result: Result<u16, DecodingError> =
            decoder.decode_u16(Fixed4Byte, &mut &[0x00u8, 0x01, 0x00][..], 0x00);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_varint() {
        let decoder: Decoder = Decoder::default();
        let result: u16 = decoder.decode_u16(VarInt, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_varint_multi_byte() {
        let decoder: Decoder = Decoder::default();
        // varint 300 = [0xAC, 0x02]
        let result: u16 = decoder
            .decode_u16(VarInt, &mut &[0x02u8][..], 0xAC)
            .unwrap();
        assert_eq!(result, 300);
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<u16, DecodingError> =
            decoder.decode_u16(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(result, Err(DecodingError::InvalidWireType(LengthPrefixed))));
    }
}
