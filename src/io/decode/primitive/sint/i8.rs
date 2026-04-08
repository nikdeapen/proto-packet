use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `i8`

    /// Decodes an `i8` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_i8<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i8, DecodingError>
    where
        R: Read,
    {
        let result: Result<i8, std::num::TryFromIntError> = match wire {
            Fixed1Byte => Ok(first as i8),
            Fixed2Byte => i8::try_from(self.decode_i16(wire, r, first)?),
            Fixed4Byte => i8::try_from(self.decode_i32(wire, r, first)?),
            Fixed8Byte => i8::try_from(self.decode_i64(wire, r, first)?),
            Fixed16Byte => i8::try_from(self.decode_i128(wire, r, first)?),
            VarInt => i8::try_from(self.decode_i32(wire, r, first)?),
            _ => {
                return Err(InvalidWireType {
                    semantic: "i8",
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

    /// A comparable representation of a [Decoder::decode_i8] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(i8),
        ValueOutOfRange,
        InvalidWireType,
    }

    impl From<Result<i8, DecodingError>> for Outcome {
        fn from(result: Result<i8, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::ValueOutOfRange) => Self::ValueOutOfRange,
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_i8() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed1Byte
            (Fixed1Byte, 0, &[], Outcome::Ok(0)),
            (Fixed1Byte, 42, &[], Outcome::Ok(42)),
            (Fixed1Byte, 0xFF, &[], Outcome::Ok(-1)),
            (Fixed1Byte, 0x7F, &[], Outcome::Ok(i8::MAX)),
            (Fixed1Byte, 0x80, &[], Outcome::Ok(i8::MIN)),
            // Fixed2Byte
            (Fixed2Byte, 42, &[0], Outcome::Ok(42)),
            (Fixed2Byte, 0xFF, &[0xFF], Outcome::Ok(-1)),
            (Fixed2Byte, 0x80, &[0x00], Outcome::ValueOutOfRange),
            (Fixed2Byte, 0x7F, &[0xFF], Outcome::ValueOutOfRange),
            // Fixed4Byte
            (Fixed4Byte, 42, &[0, 0, 0], Outcome::Ok(42)),
            (Fixed4Byte, 0xFF, &[0xFF, 0xFF, 0xFF], Outcome::Ok(-1)),
            (Fixed4Byte, 0x00, &[0x01, 0, 0], Outcome::ValueOutOfRange),
            // Fixed8Byte
            (Fixed8Byte, 42, &[0, 0, 0, 0, 0, 0, 0], Outcome::Ok(42)),
            (Fixed8Byte, 0xFF, &[0xFF; 7], Outcome::Ok(-1)),
            // Fixed16Byte
            (Fixed16Byte, 42, &[0; 15], Outcome::Ok(42)),
            (Fixed16Byte, 0xFF, &[0xFF; 15], Outcome::Ok(-1)),
            // VarInt (zigzag)
            (VarInt, 0, &[], Outcome::Ok(0)),
            (VarInt, 1, &[], Outcome::Ok(-1)),
            (VarInt, 2, &[], Outcome::Ok(1)),
            (VarInt, 0x54, &[], Outcome::Ok(42)),
            (VarInt, 0xFE, &[0x01], Outcome::Ok(i8::MAX)),
            (VarInt, 0xFF, &[0x01], Outcome::Ok(i8::MIN)),
            (VarInt, 0x80, &[0x02], Outcome::ValueOutOfRange),
            // InvalidWireType
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_i8(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
