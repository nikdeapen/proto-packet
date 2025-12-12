use crate::io::decode::list::util::decode_generic_list;
use crate::io::{Decoder, DecodingError, ListHeader, WireType};
use crate::Packet;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Decode: `[]Packet`

    /// Decodes a `[]P` from the `Read` prefix with the `first` byte.
    pub fn decode_packet_list<R, P>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Vec<P>, DecodingError>
    where
        R: Read,
        P: Packet,
    {
        match wire {
            WireType::List => {
                let header: ListHeader =
                    ListHeader::decode_from_read_prefix_with_first_byte(r, first)
                        .map_err(DecodingError::from_list_header_error)?;
                match header.wire() {
                    WireType::List => decode_generic_list(r, header, |wire, r, first| {
                        self.decode_packet(wire, r, first)
                    }),
                    _ => Err(DecodingError::InvalidListWireType(header.wire())),
                }
            }
            _ => Err(DecodingError::InvalidWireType(wire)),
        }
    }
}
