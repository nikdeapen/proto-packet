use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt128;
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
            Fixed16Byte => u128::from_le_bytes(WireType::decode_fixed_16_byte(r, first)?),
            VarInt => VarInt128::decode_from_read_prefix_with_first_byte(r, first)
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
        let result: u128 = decoder.decode_u128(Fixed1Byte, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_fixed_4_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u128 = decoder
            .decode_u128(Fixed4Byte, &mut &[0x02u8, 0x03, 0x04][..], 0x01)
            .unwrap();
        assert_eq!(result, 0x04030201);
    }

    #[test]
    fn decode_fixed_8_byte() {
        let decoder: Decoder = Decoder::default();
        let result: u128 = decoder
            .decode_u128(
                Fixed8Byte,
                &mut &[0x02u8, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08][..],
                0x01,
            )
            .unwrap();
        assert_eq!(result, 0x0807060504030201);
    }

    #[test]
    fn decode_fixed_16_byte() {
        let decoder: Decoder = Decoder::default();
        let mut remaining: [u8; 15] = [0u8; 15];
        remaining[0] = 0x02;
        let result: u128 = decoder
            .decode_u128(Fixed16Byte, &mut &remaining[..], 0x01)
            .unwrap();
        assert_eq!(result, 0x0201);
    }

    #[test]
    fn decode_varint() {
        let decoder: Decoder = Decoder::default();
        let result: u128 = decoder.decode_u128(VarInt, &mut &[][..], 42).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<u128, DecodingError> =
            decoder.decode_u128(LengthPrefixed, &mut &[][..], 0);
        assert!(matches!(result, Err(DecodingError::InvalidWireType(LengthPrefixed))));
    }
}
