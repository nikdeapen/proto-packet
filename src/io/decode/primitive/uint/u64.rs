use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt64;
use std::io::Read;

impl Decoder {
    //! Decode: `u64`

    /// Decodes the `u64` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_u64<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<u64, DecodingError>
    where
        R: Read,
    {
        let result: Result<u64, std::num::TryFromIntError> = match wire {
            Fixed1Byte => Ok(self.decode_u8(wire, r, first)? as u64),
            Fixed2Byte => Ok(self.decode_u16(wire, r, first)? as u64),
            Fixed4Byte => Ok(self.decode_u32(wire, r, first)? as u64),
            Fixed8Byte => Ok(u64::from_le_bytes(WireType::decode_fixed_8_byte(r, first)?)),
            Fixed16Byte => u64::try_from(self.decode_u128(wire, r, first)?),
            VarInt => Ok(VarInt64::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_var_int_error)?
                .value()),
            _ => {
                return Err(InvalidWireType {
                    semantic: "u64",
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

    /// A comparable representation of a [Decoder::decode_u64] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(u64),
        ValueOutOfRange,
        InvalidWireType,
    }

    impl From<Result<u64, DecodingError>> for Outcome {
        fn from(result: Result<u64, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::ValueOutOfRange) => Self::ValueOutOfRange,
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_u64() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed1Byte (u8 → u64)
            (Fixed1Byte, 0, &[], Outcome::Ok(0)),
            (Fixed1Byte, 42, &[], Outcome::Ok(42)),
            (Fixed1Byte, 0xFF, &[], Outcome::Ok(255)),
            // Fixed2Byte
            (Fixed2Byte, 0x02, &[0x03], Outcome::Ok(0x0302)),
            // Fixed4Byte
            (
                Fixed4Byte,
                0x01,
                &[0x02, 0x03, 0x04],
                Outcome::Ok(0x04030201),
            ),
            // Fixed8Byte (direct u64)
            (Fixed8Byte, 0, &[0; 7], Outcome::Ok(0)),
            (
                Fixed8Byte,
                0x01,
                &[0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                Outcome::Ok(0x0807060504030201),
            ),
            (Fixed8Byte, 0xFF, &[0xFF; 7], Outcome::Ok(u64::MAX)),
            // Fixed16Byte (u128 → u64 with range check)
            (Fixed16Byte, 42, &[0; 15], Outcome::Ok(42)),
            (
                Fixed16Byte,
                0x00,
                &[0, 0, 0, 0, 0, 0, 0, 0x01, 0, 0, 0, 0, 0, 0, 0],
                Outcome::ValueOutOfRange,
            ),
            // VarInt
            (VarInt, 0, &[], Outcome::Ok(0)),
            (VarInt, 42, &[], Outcome::Ok(42)),
            (
                VarInt,
                0xFF,
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
                Outcome::Ok(u64::MAX),
            ),
            // InvalidWireType
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_u64(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
