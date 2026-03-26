use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt32;
use std::io::Read;

impl Decoder {
    //! Decode: `i32`

    /// Decodes an `i32` value from the `Read` prefix with the `first` byte.
    pub fn decode_i32<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i32, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_i8(wire, r, first)? as i32,
            Fixed2Byte => self.decode_i16(wire, r, first)? as i32,
            Fixed4Byte => i32::from_le_bytes(WireType::decode_fixed_4_byte(r, first)?),
            Fixed8Byte => {
                let value: i64 = self.decode_i64(wire, r, first)?;
                if value > i32::MAX as i64 || value < i32::MIN as i64 {
                    return Err(ValueOutOfRange);
                }
                value as i32
            }
            Fixed16Byte => {
                let value: i128 = self.decode_i128(wire, r, first)?;
                if value > i32::MAX as i128 || value < i32::MIN as i128 {
                    return Err(ValueOutOfRange);
                }
                value as i32
            }
            VarInt => VarInt32::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_var_int_error)?
                .to_zigzag(),
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
        let result: i32 = decoder.decode_i32(Fixed1Byte, &mut &[][..], 0xFF).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_fixed_2_byte() {
        let decoder: Decoder = Decoder::default();
        let result: i32 = decoder
            .decode_i32(Fixed2Byte, &mut &[0x03u8][..], 0x02)
            .unwrap();
        assert_eq!(result, 0x0302);
    }

    #[test]
    fn decode_fixed_4_byte() {
        let decoder: Decoder = Decoder::default();
        // i32 0x04030201 le = [0x01, 0x02, 0x03, 0x04]
        let result: i32 = decoder
            .decode_i32(Fixed4Byte, &mut &[0x02u8, 0x03, 0x04][..], 0x01)
            .unwrap();
        assert_eq!(result, 0x04030201);
    }

    #[test]
    fn decode_fixed_4_byte_negative() {
        let decoder: Decoder = Decoder::default();
        // i32 -1 le = [0xFF, 0xFF, 0xFF, 0xFF]
        let result: i32 = decoder
            .decode_i32(Fixed4Byte, &mut &[0xFFu8, 0xFF, 0xFF][..], 0xFF)
            .unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_fixed_8_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        // i64 that exceeds i32::MAX
        let value: i64 = i32::MAX as i64 + 1;
        let bytes: [u8; 8] = value.to_le_bytes();
        let result: Result<i32, DecodingError> =
            decoder.decode_i32(Fixed8Byte, &mut &bytes[1..][..], bytes[0]);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_varint() {
        let decoder: Decoder = Decoder::default();
        // zigzag: 0 encodes as 0
        let result: i32 = decoder.decode_i32(VarInt, &mut &[][..], 0).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn decode_varint_negative() {
        let decoder: Decoder = Decoder::default();
        // zigzag: -1 encodes as 1
        let result: i32 = decoder.decode_i32(VarInt, &mut &[][..], 1).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_varint_positive() {
        let decoder: Decoder = Decoder::default();
        // zigzag: 1 encodes as 2
        let result: i32 = decoder.decode_i32(VarInt, &mut &[][..], 2).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<i32, DecodingError> =
            decoder.decode_i32(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(LengthPrefixed))
        ));
    }
}
