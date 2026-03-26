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
            _ => Err(DecodingError::InvalidWireType(wire)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_string() {
        let decoder: Decoder = Decoder::default();
        // length prefix 13, then "Hello, World!"
        let data: &[u8] = b"Hello, World!";
        let result: String = decoder
            .decode_string(LengthPrefixed, &mut &data[..], 13)
            .unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn decode_empty_string() {
        let decoder: Decoder = Decoder::default();
        let result: String = decoder
            .decode_string(LengthPrefixed, &mut &[][..], 0)
            .unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn decode_invalid_utf8() {
        let decoder: Decoder = Decoder::default();
        let data: &[u8] = &[0xFF, 0xFE];
        let result: Result<String, DecodingError> =
            decoder.decode_string(LengthPrefixed, &mut &data[..], 2);
        assert!(matches!(result, Err(DecodingError::InvalidString(_))));
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<String, DecodingError> =
            decoder.decode_string(Fixed1Byte, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(Fixed1Byte))
        ));
    }
}
