use crate::io::DecodingError::Stream;
use crate::io::WireType::Fixed16Byte;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;
use uuid::Uuid;

impl Decoder {
    //! Decode: `uuid`

    /// Decodes a `uuid` value from the `Read` prefix with the `first` byte.
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
            Fixed16Byte => {
                let mut buffer: [u8; 16] = [0; 16];
                buffer[0] = first;
                r.read_exact(&mut buffer[1..]).map_err(Stream)?;
                Ok(Uuid::from_bytes(buffer))
            }
            _ => Err(DecodingError::InvalidWireType(wire)),
        }
    }
}
