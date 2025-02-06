use std::io::{Cursor, Error};

use enc::read_optional_byte;

use proto_packet::io::WireType::LengthPrefixed;
use proto_packet::Packet;

use crate::file::SequenceFile;

impl SequenceFile {
    //! Read Vec

    /// Reads the file as a vec. Returns the vec.
    pub fn read_as_vec<P>(&self) -> Result<Vec<P>, Error>
    where
        P: Packet,
    {
        // todo -- streaming
        let data: Vec<u8> = self.file.read_as_vec()?;
        let mut r: Cursor<Vec<u8>> = Cursor::new(data);
        let mut result: Vec<P> = Vec::with_capacity(1024);
        if P::wire_type() == LengthPrefixed {
            while let Some(first) = read_optional_byte(&mut r)? {
                result.push(P::decode_from_read_length_prefixed_with_first_byte(
                    first, &mut r,
                )?);
            }
        } else {
            while let Some(first) = read_optional_byte(&mut r)? {
                result.push(P::decode_from_read_prefix_with_first_byte(first, &mut r)?);
            }
        }
        Ok(result)
    }
}
