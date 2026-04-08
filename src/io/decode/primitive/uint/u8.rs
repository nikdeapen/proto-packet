use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `u8`

    /// Decodes the `u8` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_u8<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<u8, DecodingError>
    where
        R: Read,
    {
        let result: Result<u8, std::num::TryFromIntError> = match wire {
            Fixed1Byte => Ok(first),
            Fixed2Byte => u8::try_from(self.decode_u16(wire, r, first)?),
            Fixed4Byte => u8::try_from(self.decode_u32(wire, r, first)?),
            Fixed8Byte => u8::try_from(self.decode_u64(wire, r, first)?),
            Fixed16Byte => u8::try_from(self.decode_u128(wire, r, first)?),
            VarInt => u8::try_from(self.decode_u32(wire, r, first)?),
            _ => {
                return Err(InvalidWireType {
                    semantic: "u8",
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

    /// A comparable representation of a [Decoder::decode_u8] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(u8),
        ValueOutOfRange,
        InvalidWireType,
    }

    impl From<Result<u8, DecodingError>> for Outcome {
        fn from(result: Result<u8, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::ValueOutOfRange) => Self::ValueOutOfRange,
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_u8() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed1Byte (direct u8)
            (Fixed1Byte, 0, &[], Outcome::Ok(0)),
            (Fixed1Byte, 42, &[], Outcome::Ok(42)),
            (Fixed1Byte, 0xFF, &[], Outcome::Ok(u8::MAX)),
            // Fixed2Byte (u16 → u8 with range check)
            (Fixed2Byte, 42, &[0], Outcome::Ok(42)),
            (Fixed2Byte, 0xFF, &[0], Outcome::Ok(255)),
            (Fixed2Byte, 0x00, &[0x01], Outcome::ValueOutOfRange),
            // Fixed4Byte
            (Fixed4Byte, 42, &[0, 0, 0], Outcome::Ok(42)),
            (Fixed4Byte, 0x00, &[0x01, 0, 0], Outcome::ValueOutOfRange),
            // Fixed8Byte
            (Fixed8Byte, 42, &[0; 7], Outcome::Ok(42)),
            (
                Fixed8Byte,
                0x00,
                &[0x01, 0, 0, 0, 0, 0, 0],
                Outcome::ValueOutOfRange,
            ),
            // Fixed16Byte
            (Fixed16Byte, 42, &[0; 15], Outcome::Ok(42)),
            (
                Fixed16Byte,
                0x00,
                &[0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                Outcome::ValueOutOfRange,
            ),
            // VarInt
            (VarInt, 0, &[], Outcome::Ok(0)),
            (VarInt, 42, &[], Outcome::Ok(42)),
            (VarInt, 0x7F, &[], Outcome::Ok(127)),
            (VarInt, 0xFF, &[0x01], Outcome::Ok(u8::MAX)),
            (VarInt, 0x80, &[0x02], Outcome::ValueOutOfRange),
            // InvalidWireType
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_u8(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
