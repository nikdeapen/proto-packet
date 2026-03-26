use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt64;
use std::io::Read;

impl Decoder {
    //! Decode: `i64`

    /// Decodes an `i64` value from the `Read` prefix with the `first` byte.
    pub fn decode_i64<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i64, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_i8(wire, r, first)? as i64,
            Fixed2Byte => self.decode_i16(wire, r, first)? as i64,
            Fixed4Byte => self.decode_i32(wire, r, first)? as i64,
            Fixed8Byte => i64::from_le_bytes(WireType::decode_fixed_8_byte(r, first)?),
            Fixed16Byte => {
                let value: i128 = self.decode_i128(wire, r, first)?;
                if value > i64::MAX as i128 || value < i64::MIN as i128 {
                    return Err(ValueOutOfRange);
                }
                value as i64
            }
            VarInt => VarInt64::decode_from_read_prefix_with_first_byte(r, first)
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
        let result: i64 = decoder.decode_i64(Fixed1Byte, &mut &[][..], 0xFF).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_fixed_4_byte() {
        let decoder: Decoder = Decoder::default();
        let result: i64 = decoder
            .decode_i64(Fixed4Byte, &mut &[0x02u8, 0x03, 0x04][..], 0x01)
            .unwrap();
        assert_eq!(result, 0x04030201);
    }

    #[test]
    fn decode_fixed_8_byte() {
        let decoder: Decoder = Decoder::default();
        // i64 0x0807060504030201 le = [0x01, ..., 0x08]
        let result: i64 = decoder
            .decode_i64(
                Fixed8Byte,
                &mut &[0x02u8, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08][..],
                0x01,
            )
            .unwrap();
        assert_eq!(result, 0x0807060504030201);
    }

    #[test]
    fn decode_fixed_8_byte_negative() {
        let decoder: Decoder = Decoder::default();
        // i64 -1 le = [0xFF; 8]
        let result: i64 = decoder
            .decode_i64(Fixed8Byte, &mut &[0xFFu8; 7][..], 0xFF)
            .unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_fixed_16_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        let mut remaining: [u8; 15] = [0u8; 15];
        remaining[7] = 1; // sets byte 8 overall, making it > i64::MAX
        let result: Result<i64, DecodingError> =
            decoder.decode_i64(Fixed16Byte, &mut &remaining[..], 0);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_varint_negative() {
        let decoder: Decoder = Decoder::default();
        // zigzag: -1 encodes as 1
        let result: i64 = decoder.decode_i64(VarInt, &mut &[][..], 1).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn decode_varint_positive() {
        let decoder: Decoder = Decoder::default();
        // zigzag: 1 encodes as 2
        let result: i64 = decoder.decode_i64(VarInt, &mut &[][..], 2).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<i64, DecodingError> =
            decoder.decode_i64(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(LengthPrefixed))
        ));
    }
}
