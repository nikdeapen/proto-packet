use std::io::{Cursor, Error};

use enc::EncodeToWrite;
use enc::var_int::VarIntSize;
use proto_packet::io::WireType::LengthPrefixed;
use proto_packet::Packet;

use crate::file::SequenceFile;

impl SequenceFile {
    //! Write slice

    /// Writes the `slice`. Returns the number of bytes written.
    pub fn write_slice<P>(&self, slice: &[P]) -> Result<usize, Error>
    where
        P: Packet,
    {
        // todo -- streaming
        let buffer: Vec<u8> = Vec::default();
        let mut buffer: Cursor<Vec<u8>> = Cursor::new(buffer);

        if P::wire_type() == LengthPrefixed {
            for packet in slice {
                VarIntSize::from(packet.encoded_len()?).encode_to_write(&mut buffer)?;
                packet.encode_to_write(&mut buffer)?;
            }
        } else {
            for packet in slice {
                packet.encode_to_write(&mut buffer)?;
            }
        }

        let buffer: Vec<u8> = buffer.into_inner();
        self.file.write_data(buffer.as_slice())?;

        Ok(buffer.len())
    }
}
