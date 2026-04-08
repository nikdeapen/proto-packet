use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarInt128;
use std::io::Read;

impl Decoder {
    //! Decode: `i128`

    /// Decodes an `i128` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_i128<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<i128, DecodingError>
    where
        R: Read,
    {
        match wire {
            Fixed1Byte => Ok(self.decode_i8(wire, r, first)? as i128),
            Fixed2Byte => Ok(self.decode_i16(wire, r, first)? as i128),
            Fixed4Byte => Ok(self.decode_i32(wire, r, first)? as i128),
            Fixed8Byte => Ok(self.decode_i64(wire, r, first)? as i128),
            Fixed16Byte => Ok(i128::from_le_bytes(WireType::decode_fixed_16_byte(
                r, first,
            )?)),
            VarInt => Ok(VarInt128::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_var_int_error)?
                .to_zigzag()),
            _ => Err(InvalidWireType {
                semantic: "i128",
                wire,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError, WireType};

    /// A comparable representation of a [Decoder::decode_i128] result.
    ///
    /// `i128` is the widest type, so no input can be out of range — only [Outcome::Ok] and
    /// [Outcome::InvalidWireType] are reachable.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(i128),
        InvalidWireType,
    }

    impl From<Result<i128, DecodingError>> for Outcome {
        fn from(result: Result<i128, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_i128() {
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
            // Fixed8Byte (i64 sign-extends)
            (
                Fixed8Byte,
                0x01,
                &[0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                Outcome::Ok(0x0807060504030201),
            ),
            (Fixed8Byte, 0xFF, &[0xFF; 7], Outcome::Ok(-1)),
            // Fixed16Byte (direct i128)
            (
                Fixed16Byte,
                0x01,
                &[0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                Outcome::Ok(0x0201),
            ),
            (Fixed16Byte, 0xFF, &[0xFF; 15], Outcome::Ok(-1)),
            (
                Fixed16Byte,
                0xFF,
                &[
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0x7F,
                ],
                Outcome::Ok(i128::MAX),
            ),
            (
                Fixed16Byte,
                0x00,
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80],
                Outcome::Ok(i128::MIN),
            ),
            // VarInt (zigzag)
            (VarInt, 0, &[], Outcome::Ok(0)),
            (VarInt, 1, &[], Outcome::Ok(-1)),
            (VarInt, 2, &[], Outcome::Ok(1)),
            // InvalidWireType
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_i128(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
