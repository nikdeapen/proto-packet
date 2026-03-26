use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt64;
use std::io::Read;

impl Decoder {
    //! Decode: `u64`

    /// Decodes the `u64` value from the `Read` prefix with the `first` byte.
    pub fn decode_u64<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<u64, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => self.decode_u8(wire, r, first)? as u64,
            Fixed2Byte => self.decode_u16(wire, r, first)? as u64,
            Fixed4Byte => self.decode_u32(wire, r, first)? as u64,
            Fixed8Byte => u64::from_le_bytes(WireType::decode_fixed_8_byte(r, first)?),
            Fixed16Byte => {
                let value: u128 = self.decode_u128(wire, r, first)?;
                if value > u64::MAX as u128 {
                    return Err(ValueOutOfRange);
                }
                value as u64
            }
            VarInt => VarInt64::decode_from_read_prefix_with_first_byte(r, first)
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
        let result: u64 = decoder.decode_u64(Fixed1Byte, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_2_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u64 = decoder
            .decode_u64(Fixed2Byte, &mut &[0x03u8][..], 0x02)
            .unwrap();
        assert_eq!(result, 0x0302);
    }

    #[test]
    fn decode_fixed_4_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u64 = decoder
            .decode_u64(Fixed4Byte, &mut &[0x02u8, 0x03, 0x04][..], 0x01)
            .unwrap();
        assert_eq!(result, 0x04030201);
    }

    #[test]
    fn decode_fixed_8_byte() {
        let decoder: Decoder = Decoder::default();
        // 0x0807060504030201 le = [0x01, 0x02, ..., 0x08]
        let result: u64 = decoder
            .decode_u64(
                Fixed8Byte,
                &mut &[0x02u8, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08][..],
                0x01,
            )
            .unwrap();
        assert_eq!(result, 0x0807060504030201);
    }

    #[test]
    fn decode_fixed_16_byte_out_of_range() {
        let decoder: Decoder = Decoder::default();
        let mut remaining: [u8; 15] = [0u8; 15];
        remaining[7] = 1; // sets byte 8 (index 8 overall) making it > u64::MAX
        let result: Result<u64, DecodingError> =
            decoder.decode_u64(Fixed16Byte, &mut &remaining[..], 0);
        assert!(matches!(result, Err(DecodingError::ValueOutOfRange)));
    }

    #[test]
    fn decode_varint() {
        let decoder: Decoder = Decoder::default();
        let result: u64 = decoder.decode_u64(VarInt, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<u64, DecodingError> =
            decoder.decode_u64(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(LengthPrefixed))
        ));
    }
}
