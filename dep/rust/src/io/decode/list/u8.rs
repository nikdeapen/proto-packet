use crate::io::decode::list::util::decode_generic_list;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, ListHeader, WireType};
use enc::var_int::VarIntSize;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `[]u8`

    /// Decodes the `[]u8` value from the `Read` prefix with the `first` byte.
    pub fn decode_u8_list<R>(
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
                    .map_err(DecodingError::from_length_prefix_error)?
                    .value();
                let mut buffer: Vec<u8> = vec![0u8; prefix]; // todo -- performance & security
                r.read_exact(buffer.as_mut_slice())
                    .map_err(DecodingError::Stream)?;
                Ok(buffer)
            }
            List => {
                let header: ListHeader =
                    ListHeader::decode_from_read_prefix_with_first_byte(r, first)
                        .map_err(DecodingError::from_list_header_error)?;
                match header.wire() {
                    WireType::Fixed1Byte => {
                        let mut buffer: Vec<u8> = vec![0u8; header.size()]; // todo -- performance & security
                        r.read_exact(buffer.as_mut_slice())?;
                        Ok(buffer)
                    }
                    _ => decode_generic_list(r, header, |wire, r, first| {
                        self.decode_u8(wire, r, first)
                    }),
                }
            }
            wire => Err(DecodingError::InvalidWireType(wire)),
        }
    }
}
