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

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_fixed_1_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u8 = decoder.decode_u8(Fixed1Byte, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_2_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u8 = decoder.decode_u8(Fixed2Byte, &mut &[0u8][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_2_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        let result: Result<u8, DecodingError> = decoder.decode_u8(Fixed2Byte, &mut &[1u8][..], 0);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_fixed_4_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u8 = decoder
            .decode_u8(Fixed4Byte, &mut &[0u8, 0, 0][..], 42)
            .unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_4_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        let result: Result<u8, DecodingError> =
            decoder.decode_u8(Fixed4Byte, &mut &[1u8, 0, 0][..], 0);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_varint() {
        let decoder: Decoder = Decoder::default();
        let result: u8 = decoder.decode_u8(VarInt, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_varint_out_of_range() {
        let decoder: Decoder = Decoder::default();
        // varint 256 = [0x80, 0x02]
        let result: Result<u8, DecodingError> = decoder.decode_u8(VarInt, &mut &[0x02u8][..], 0x80);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<u8, DecodingError> = decoder.decode_u8(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(LengthPrefixed))
        ));
    }
}
