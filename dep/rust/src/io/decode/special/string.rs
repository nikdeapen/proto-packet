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
                let value: String = String::from_utf8(value).map_err(|e| InvalidString(e))?;
                Ok(value)
            }
            _ => Err(DecodingError::InvalidWireType(wire)),
        }
    }
}
