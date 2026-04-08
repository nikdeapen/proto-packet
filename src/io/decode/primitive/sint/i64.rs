use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt64;
use std::io::Read;

impl Decoder {
    //! Decode: `i64`

    /// Decodes an `i64` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_i64<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i64, DecodingError>
    where
        R: Read,
    {
        let result: Result<i64, std::num::TryFromIntError> = match wire {
            Fixed1Byte => Ok(self.decode_i8(wire, r, first)? as i64),
            Fixed2Byte => Ok(self.decode_i16(wire, r, first)? as i64),
            Fixed4Byte => Ok(self.decode_i32(wire, r, first)? as i64),
            Fixed8Byte => Ok(i64::from_le_bytes(WireType::decode_fixed_8_byte(r, first)?)),
            Fixed16Byte => i64::try_from(self.decode_i128(wire, r, first)?),
            VarInt => Ok(VarInt64::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_var_int_error)?
                .to_zigzag()),
            _ => {
                return Err(InvalidWireType {
                    semantic: "i64",
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

    /// A comparable representation of a [Decoder::decode_i64] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(i64),
        ValueOutOfRange,
        InvalidWireType,
    }

    impl From<Result<i64, DecodingError>> for Outcome {
        fn from(result: Result<i64, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::ValueOutOfRange) => Self::ValueOutOfRange,
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_i64() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed1Byte (i8 sign-extends)
            (Fixed1Byte, 0, &[], Outcome::Ok(0)),
            (Fixed1Byte, 42, &[], Outcome::Ok(42)),
            (Fixed1Byte, 0xFF, &[], Outcome::Ok(-1)),
            // Fixed2Byte (i16 sign-extends)
            (Fixed2Byte, 0x02, &[0x03], Outcome::Ok(0x0302)),
            (Fixed2Byte, 0xFF, &[0xFF], Outcome::Ok(-1)),
            // Fixed4Byte (i32 sign-extends)
            (
                Fixed4Byte,
                0x01,
                &[0x02, 0x03, 0x04],
                Outcome::Ok(0x04030201),
            ),
            (Fixed4Byte, 0xFF, &[0xFF, 0xFF, 0xFF], Outcome::Ok(-1)),
            // Fixed8Byte (direct i64)
            (
                Fixed8Byte,
                0x01,
                &[0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                Outcome::Ok(0x0807060504030201),
            ),
            (Fixed8Byte, 0xFF, &[0xFF; 7], Outcome::Ok(-1)),
            (
                Fixed8Byte,
                0xFF,
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
                Outcome::Ok(i64::MAX),
            ),
            (
                Fixed8Byte,
                0x00,
                &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
                Outcome::Ok(i64::MIN),
            ),
            // Fixed16Byte (i128 → i64 with range check)
            (Fixed16Byte, 42, &[0; 15], Outcome::Ok(42)),
            (Fixed16Byte, 0xFF, &[0xFF; 15], Outcome::Ok(-1)),
            (
                Fixed16Byte,
                0x00,
                &[0, 0, 0, 0, 0, 0, 0x80, 0, 0, 0, 0, 0, 0, 0, 0],
                Outcome::ValueOutOfRange,
            ),
            (
                Fixed16Byte,
                0xFF,
                &[
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF,
                ],
                Outcome::ValueOutOfRange,
            ),
            // VarInt (zigzag)
            (VarInt, 0, &[], Outcome::Ok(0)),
            (VarInt, 1, &[], Outcome::Ok(-1)),
            (VarInt, 2, &[], Outcome::Ok(1)),
            (
                VarInt,
                0xFE,
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
                Outcome::Ok(i64::MAX),
            ),
            (
                VarInt,
                0xFF,
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
                Outcome::Ok(i64::MIN),
            ),
            // InvalidWireType
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_i64(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
