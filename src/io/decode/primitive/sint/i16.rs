use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `i16`

    /// Decodes an `i16` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_i16<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<i16, DecodingError>
    where
        R: Read,
    {
        let result: Result<i16, std::num::TryFromIntError> = match wire {
            Fixed1Byte => Ok(self.decode_i8(wire, r, first)? as i16),
            Fixed2Byte => Ok(i16::from_le_bytes(WireType::decode_fixed_2_byte(r, first)?)),
            Fixed4Byte => i16::try_from(self.decode_i32(wire, r, first)?),
            Fixed8Byte => i16::try_from(self.decode_i64(wire, r, first)?),
            Fixed16Byte => i16::try_from(self.decode_i128(wire, r, first)?),
            VarInt => i16::try_from(self.decode_i32(wire, r, first)?),
            _ => {
                return Err(InvalidWireType {
                    semantic: "i16",
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

    /// A comparable representation of a [Decoder::decode_i16] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(i16),
        ValueOutOfRange,
        InvalidWireType,
    }

    impl From<Result<i16, DecodingError>> for Outcome {
        fn from(result: Result<i16, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::ValueOutOfRange) => Self::ValueOutOfRange,
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_i16() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed1Byte (i8 sign-extends to i16)
            (Fixed1Byte, 0, &[], Outcome::Ok(0)),
            (Fixed1Byte, 42, &[], Outcome::Ok(42)),
            (Fixed1Byte, 0xFF, &[], Outcome::Ok(-1)),
            (Fixed1Byte, 0x7F, &[], Outcome::Ok(127)),
            (Fixed1Byte, 0x80, &[], Outcome::Ok(-128)),
            // Fixed2Byte (direct i16)
            (Fixed2Byte, 0, &[0], Outcome::Ok(0)),
            (Fixed2Byte, 0x02, &[0x03], Outcome::Ok(0x0302)),
            (Fixed2Byte, 0xFF, &[0xFF], Outcome::Ok(-1)),
            (Fixed2Byte, 0xFF, &[0x7F], Outcome::Ok(i16::MAX)),
            (Fixed2Byte, 0x00, &[0x80], Outcome::Ok(i16::MIN)),
            // Fixed4Byte (i32 → i16 with range check)
            (Fixed4Byte, 42, &[0, 0, 0], Outcome::Ok(42)),
            (Fixed4Byte, 0xFF, &[0xFF, 0xFF, 0xFF], Outcome::Ok(-1)),
            (
                Fixed4Byte,
                0x00,
                &[0x80, 0x00, 0x00],
                Outcome::ValueOutOfRange,
            ),
            (
                Fixed4Byte,
                0xFF,
                &[0x7F, 0xFF, 0xFF],
                Outcome::ValueOutOfRange,
            ),
            // Fixed8Byte
            (Fixed8Byte, 42, &[0; 7], Outcome::Ok(42)),
            (Fixed8Byte, 0xFF, &[0xFF; 7], Outcome::Ok(-1)),
            (
                Fixed8Byte,
                0x00,
                &[0x80, 0, 0, 0, 0, 0, 0],
                Outcome::ValueOutOfRange,
            ),
            // Fixed16Byte
            (Fixed16Byte, 42, &[0; 15], Outcome::Ok(42)),
            (Fixed16Byte, 0xFF, &[0xFF; 15], Outcome::Ok(-1)),
            // VarInt (zigzag, decoded via VarInt32)
            (VarInt, 0, &[], Outcome::Ok(0)),
            (VarInt, 1, &[], Outcome::Ok(-1)),
            (VarInt, 2, &[], Outcome::Ok(1)),
            (VarInt, 0x54, &[], Outcome::Ok(42)),
            (VarInt, 0xFE, &[0xFF, 0x03], Outcome::Ok(i16::MAX)),
            (VarInt, 0xFF, &[0xFF, 0x03], Outcome::Ok(i16::MIN)),
            (VarInt, 0x80, &[0x80, 0x04], Outcome::ValueOutOfRange),
            // InvalidWireType
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_i16(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
