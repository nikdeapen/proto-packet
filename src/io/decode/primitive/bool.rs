use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `bool`

    /// Decodes the `bool` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_bool<R>(
        &self,
        wire: WireType,
        _r: &mut R, // this parameter is present for code-generation consistency
        first: u8,
    ) -> Result<bool, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => match first {
                0 => false,
                1 => true,
                _ => return Err(InvalidBool(first)),
            },
            _ => {
                return Err(InvalidWireType {
                    semantic: "bool",
                    wire,
                });
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError, WireType};

    /// A comparable representation of a [Decoder::decode_bool] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(bool),
        InvalidBool(u8),
        InvalidWireType,
    }

    impl From<Result<bool, DecodingError>> for Outcome {
        fn from(result: Result<bool, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::InvalidBool(byte)) => Self::InvalidBool(byte),
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_bool() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed1Byte
            (Fixed1Byte, 0, &[], Outcome::Ok(false)),
            (Fixed1Byte, 1, &[], Outcome::Ok(true)),
            (Fixed1Byte, 2, &[], Outcome::InvalidBool(2)),
            (Fixed1Byte, 0xFF, &[], Outcome::InvalidBool(0xFF)),
            // InvalidWireType
            (VarInt, 0, &[], Outcome::InvalidWireType),
            (Fixed2Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed4Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed8Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed16Byte, 0, &[], Outcome::InvalidWireType),
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_bool(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
