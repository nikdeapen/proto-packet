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

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_fixed_1_byte() {
        let decoder: Decoder = Decoder::default();
        let result: i8 = decoder.decode_i8(Fixed1Byte, &mut &[][..], 0xFF).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_fixed_1_byte_positive() {
        let decoder: Decoder = Decoder::default();
        let result: i8 = decoder.decode_i8(Fixed1Byte, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_2_byte() {
        let decoder: Decoder = Decoder::default();
        // i8 42 in i16 le = [42, 0]
        let result: i8 = decoder.decode_i8(Fixed2Byte, &mut &[0u8][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_2_byte_negative() {
        let decoder: Decoder = Decoder::default();
        // i16 -1 le = [0xFF, 0xFF]
        let result: i8 = decoder
            .decode_i8(Fixed2Byte, &mut &[0xFFu8][..], 0xFF)
            .unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_fixed_2_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        // i16 128 le = [0x80, 0x00] > i8::MAX
        let result: Result<i8, DecodingError> =
            decoder.decode_i8(Fixed2Byte, &mut &[0x00u8][..], 0x80);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_varint() {
        let decoder: Decoder = Decoder::default();
        // zigzag: 42 encodes as 84 = 0x54
        let result: i8 = decoder.decode_i8(VarInt, &mut &[][..], 0x54).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_varint_negative() {
        let decoder: Decoder = Decoder::default();
        // zigzag: -1 encodes as 1
        let result: i8 = decoder.decode_i8(VarInt, &mut &[][..], 1).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<i8, DecodingError> = decoder.decode_i8(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(LengthPrefixed))
        ));
    }
}
