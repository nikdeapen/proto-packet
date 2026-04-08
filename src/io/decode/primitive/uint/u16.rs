use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt16;
use std::io::Read;

impl Decoder {
    //! Decode: `u16`

    /// Decodes the `u16` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_u16<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<u16, DecodingError>
    where
        R: Read,
    {
        let result: Result<u16, std::num::TryFromIntError> = match wire {
            Fixed1Byte => Ok(self.decode_u8(wire, r, first)? as u16),
            Fixed2Byte => Ok(u16::from_le_bytes(WireType::decode_fixed_2_byte(r, first)?)),
            Fixed4Byte => u16::try_from(self.decode_u32(wire, r, first)?),
            Fixed8Byte => u16::try_from(self.decode_u64(wire, r, first)?),
            Fixed16Byte => u16::try_from(self.decode_u128(wire, r, first)?),
            VarInt => Ok(VarInt16::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_var_int_error)?
                .value()),
            _ => {
                return Err(InvalidWireType {
                    semantic: "u16",
                    wire,
                });
            }
        };
        result.map_err(|_| ValueOutOfRange)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError, WireType};

    /// A comparable representation of a [Decoder::decode_u16] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(u16),
        ValueOutOfRange,
        InvalidWireType,
    }

    impl From<Result<u16, DecodingError>> for Outcome {
        fn from(result: Result<u16, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::ValueOutOfRange) => Self::ValueOutOfRange,
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_u16() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed1Byte (u8 → u16)
            (Fixed1Byte, 0, &[], Outcome::Ok(0)),
            (Fixed1Byte, 42, &[], Outcome::Ok(42)),
            (Fixed1Byte, 0xFF, &[], Outcome::Ok(255)),
            // Fixed2Byte (direct u16)
            (Fixed2Byte, 0, &[0], Outcome::Ok(0)),
            (Fixed2Byte, 0x02, &[0x03], Outcome::Ok(0x0302)),
            (Fixed2Byte, 0xFF, &[0xFF], Outcome::Ok(u16::MAX)),
            // Fixed4Byte (u32 → u16 with range check)
            (Fixed4Byte, 42, &[0, 0, 0], Outcome::Ok(42)),
            (
                Fixed4Byte,
                0x00,
                &[0x00, 0x01, 0x00],
                Outcome::ValueOutOfRange,
            ),
            // Fixed8Byte
            (Fixed8Byte, 42, &[0; 7], Outcome::Ok(42)),
            (
                Fixed8Byte,
                0x00,
                &[0x00, 0x01, 0, 0, 0, 0, 0],
                Outcome::ValueOutOfRange,
            ),
            // Fixed16Byte
            (Fixed16Byte, 42, &[0; 15], Outcome::Ok(42)),
            // VarInt (decoded via VarInt16)
            (VarInt, 0, &[], Outcome::Ok(0)),
            (VarInt, 42, &[], Outcome::Ok(42)),
            (VarInt, 0xAC, &[0x02], Outcome::Ok(300)),
            (VarInt, 0xFF, &[0xFF, 0x03], Outcome::Ok(u16::MAX)),
            (VarInt, 0x80, &[0x80, 0x04], Outcome::ValueOutOfRange),
            // InvalidWireType
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_u16(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
