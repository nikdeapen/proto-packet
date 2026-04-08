use crate::io::DecodingError::InvalidString;
use crate::io::WireType::LengthPrefixed;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `string`

    /// Decodes a `string` value from the `Read` prefix with the `first` byte.
    pub fn decode_string<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<String, DecodingError>
    where
        R: Read,
    {
        match wire {
            LengthPrefixed => {
                let value: Vec<u8> = WireType::decode_length_prefixed_bytes(r, first)?;
                let value: String = String::from_utf8(value).map_err(InvalidString)?;
                Ok(value)
            }
            _ => Err(DecodingError::InvalidWireType {
                semantic: "String",
                wire,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError, WireType};

    /// A comparable representation of a [Decoder::decode_string] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(String),
        InvalidString,
        InvalidWireType,
    }

    impl From<Result<String, DecodingError>> for Outcome {
        fn from(result: Result<String, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::InvalidString(_)) => Self::InvalidString,
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_string() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // LengthPrefixed
            (LengthPrefixed, 0, &[], Outcome::Ok(String::new())),
            (
                LengthPrefixed,
                13,
                b"Hello, World!",
                Outcome::Ok("Hello, World!".to_string()),
            ),
            (
                LengthPrefixed,
                5,
                b"hello",
                Outcome::Ok("hello".to_string()),
            ),
            (LengthPrefixed, 2, &[0xFF, 0xFE], Outcome::InvalidString),
            // InvalidWireType
            (Fixed1Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed2Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed4Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed8Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed16Byte, 0, &[], Outcome::InvalidWireType),
            (VarInt, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_string(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
