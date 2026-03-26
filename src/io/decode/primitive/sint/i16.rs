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

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_fixed_1_byte() {
        let decoder: Decoder = Decoder::default();
        // i8 -1 (0xFF) sign-extends to i16 -1
        let result: i16 = decoder.decode_i16(Fixed1Byte, &mut &[][..], 0xFF).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_fixed_2_byte() {
        let decoder: Decoder = Decoder::default();
        // i16 0x0302 le = [0x02, 0x03]
        let result: i16 = decoder
            .decode_i16(Fixed2Byte, &mut &[0x03u8][..], 0x02)
            .unwrap();
        assert_eq!(result, 0x0302);
    }

    #[test]
    fn decode_fixed_2_byte_negative() {
        let decoder: Decoder = Decoder::default();
        // i16 -1 le = [0xFF, 0xFF]
        let result: i16 = decoder
            .decode_i16(Fixed2Byte, &mut &[0xFFu8][..], 0xFF)
            .unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_fixed_4_byte() {
        let decoder: Decoder = Decoder::default();
        let result: i16 = decoder
            .decode_i16(Fixed4Byte, &mut &[0u8, 0, 0][..], 42)
            .unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_4_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        // i32 32768 > i16::MAX, le = [0x00, 0x80, 0x00, 0x00]
        let result: Result<i16, DecodingError> =
            decoder.decode_i16(Fixed4Byte, &mut &[0x80u8, 0x00, 0x00][..], 0x00);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_varint() {
        let decoder: Decoder = Decoder::default();
        // zigzag: 42 encodes as 84 = 0x54
        let result: i16 = decoder.decode_i16(VarInt, &mut &[][..], 0x54).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_varint_negative() {
        let decoder: Decoder = Decoder::default();
        // zigzag: -1 encodes as 1
        let result: i16 = decoder.decode_i16(VarInt, &mut &[][..], 1).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<i16, DecodingError> =
            decoder.decode_i16(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(LengthPrefixed))
        ));
    }
}
