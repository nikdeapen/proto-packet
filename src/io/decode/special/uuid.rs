use crate::io::WireType::Fixed16Byte;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;
use uuid::Uuid;

impl Decoder {
    //! Decode: `uuid`

    /// Decodes a `uuid` value from the `Read` prefix with the `first` byte.
    #[inline]
    pub fn decode_uuid<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Uuid, DecodingError>
    where
        R: Read,
    {
        match wire {
            Fixed16Byte => Ok(Uuid::from_bytes(WireType::decode_fixed_16_byte(r, first)?)),
            _ => Err(DecodingError::InvalidWireType {
                semantic: "Uuid",
                wire,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError, WireType};
    use uuid::Uuid;

    /// A comparable representation of a [Decoder::decode_uuid] result.
    #[derive(Debug, PartialEq, Eq)]
    enum Outcome {
        Ok(Uuid),
        InvalidWireType,
    }

    impl From<Result<Uuid, DecodingError>> for Outcome {
        fn from(result: Result<Uuid, DecodingError>) -> Self {
            match result {
                Ok(value) => Self::Ok(value),
                Err(DecodingError::InvalidWireType { .. }) => Self::InvalidWireType,
                Err(error) => panic!("unexpected error: {:?}", error),
            }
        }
    }

    #[test]
    fn decode_uuid() {
        let cases: &[(WireType, u8, &[u8], Outcome)] = &[
            // Fixed16Byte
            (Fixed16Byte, 0, &[0u8; 15], Outcome::Ok(Uuid::nil())),
            (
                Fixed16Byte,
                0,
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                Outcome::Ok(Uuid::from_bytes([
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                ])),
            ),
            (
                Fixed16Byte,
                0xFF,
                &[0xFF; 15],
                Outcome::Ok(Uuid::from_bytes([0xFF; 16])),
            ),
            // InvalidWireType
            (Fixed1Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed2Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed4Byte, 0, &[], Outcome::InvalidWireType),
            (Fixed8Byte, 0, &[], Outcome::InvalidWireType),
            (VarInt, 0, &[], Outcome::InvalidWireType),
            (LengthPrefixed, 0, &[], Outcome::InvalidWireType),
            (List, 0, &[], Outcome::InvalidWireType),
        ];

        let decoder: Decoder = Decoder::default();
        for (wire, first, rest, expected) in cases {
            let mut input: &[u8] = rest;
            let actual: Outcome = decoder.decode_uuid(*wire, &mut input, *first).into();
            assert_eq!(
                actual, *expected,
                "wire={wire:?} first={first:#x} rest={rest:?}"
            );
        }
    }
}
