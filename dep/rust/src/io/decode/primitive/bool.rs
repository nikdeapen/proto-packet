use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `bool`

    /// Decodes a `bool` value from the `Read` prefix with the `first` byte.
    pub fn decode_bool<R>(
        &self,
        wire: WireType,
        _r: &mut R, // todo -- param for consistency & code generation
        first: u8,
    ) -> Result<bool, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => {
                if first == 0 {
                    false
                } else if first == 1 {
                    true
                } else {
                    return Err(InvalidEncodedBoolean(first));
                }
            }
            _ => return Err(InvalidWireType(wire)),
        })
    }
}
