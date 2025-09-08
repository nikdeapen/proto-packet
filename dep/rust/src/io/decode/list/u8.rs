use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, ListHeader, WireType};
use enc::var_int::VarIntSize;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `[]u8`

    /// Decodes a `[]u8` value from the `Read` prefix with the `first` byte.
    pub fn decode_u8_slice<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Vec<u8>, DecodingError>
    where
        R: Read,
    {
        match wire {
            LengthPrefixed => {
                let prefix: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
                    .map_err(|e| DecodingError::error_reading_length_prefix(e))?
                    .value();
                let mut buffer: Vec<u8> = vec![0u8; prefix];
                r.read_exact(&mut buffer)
                    .map_err(|e| DecodingError::Source(e))?;
                Ok(buffer)
            }
            List => {
                let header: ListHeader =
                    ListHeader::decode_from_read_prefix_with_first_byte(r, first)
                        .map_err(|e| DecodingError::from_list_header(e))?;
                self.decode_list_value(header, r, |r, first| {
                    self.decode_u8(header.wire_type(), r, first)
                })
            }
            _ => Err(DecodingError::InvalidWireType(wire)),
        }
    }
}
