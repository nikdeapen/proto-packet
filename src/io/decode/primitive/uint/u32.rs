use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt32;
use std::io::Read;

impl Decoder {
    //! Decode: `u32`

    /// Decodes the `u32` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_u32<R>(&self, wire: WireType, r: &mut R, first: u8) -> Result<u32, DecodingError>
    where
        R: Read,
    {
        let result: Result<u32, std::num::TryFromIntError> = match wire {
            Fixed1Byte => Ok(self.decode_u8(wire, r, first)? as u32),
            Fixed2Byte => Ok(self.decode_u16(wire, r, first)? as u32),
            Fixed4Byte => Ok(u32::from_le_bytes(WireType::decode_fixed_4_byte(r, first)?)),
            Fixed8Byte => u32::try_from(self.decode_u64(wire, r, first)?),
            Fixed16Byte => u32::try_from(self.decode_u128(wire, r, first)?),
            VarInt => Ok(VarInt32::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_var_int_error)?
                .value()),
            _ => {
                return Err(InvalidWireType {
                    semantic: "u32",
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

    /// A comparable representation of a [Decoder::decode_u32] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(u32),
        ValueOutOfRange,
        InvalidWireType,
    }

    impl From<Result<u32, DecodingError>> for Outcome {
        fn from(result: Result<u32, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::ValueOutOfRange) => Self::ValueOutOfRange,
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_u32() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed1Byte (u8 → u32)
            (Fixed1Byte, 0, &[], Outcome::Ok(0)),
            (Fixed1Byte, 42, &[], Outcome::Ok(42)),
            (Fixed1Byte, 0xFF, &[], Outcome::Ok(255)),
            // Fixed2Byte (u16 → u32)
            (Fixed2Byte, 0x02, &[0x03], Outcome::Ok(0x0302)),
            (Fixed2Byte, 0xFF, &[0xFF], Outcome::Ok(65535)),
            // Fixed4Byte (direct u32)
            (Fixed4Byte, 0, &[0, 0, 0], Outcome::Ok(0)),
            (
                Fixed4Byte,
                0x01,
                &[0x02, 0x03, 0x04],
                Outcome::Ok(0x04030201),
            ),
            (Fixed4Byte, 0xFF, &[0xFF, 0xFF, 0xFF], Outcome::Ok(u32::MAX)),
            // Fixed8Byte (u64 → u32 with range check)
            (Fixed8Byte, 42, &[0, 0, 0, 0, 0, 0, 0], Outcome::Ok(42)),
            (
                Fixed8Byte,
                0x00,
                &[0, 0, 0, 0x01, 0, 0, 0],
                Outcome::ValueOutOfRange,
            ),
            // Fixed16Byte
            (Fixed16Byte, 42, &[0; 15], Outcome::Ok(42)),
            // VarInt
            (VarInt, 0, &[], Outcome::Ok(0)),
            (VarInt, 42, &[], Outcome::Ok(42)),
            (VarInt, 0xAC, &[0x02], Outcome::Ok(300)),
            (
                VarInt,
                0xFF,
                &[0xFF, 0xFF, 0xFF, 0x0F],
                Outcome::Ok(u32::MAX),
            ),
            (
                VarInt,
                0x80,
                &[0x80, 0x80, 0x80, 0x10],
                Outcome::ValueOutOfRange,
            ),
            // InvalidWireType
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_u32(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
