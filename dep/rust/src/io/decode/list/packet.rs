use crate::io::DecodingError::InvalidListWire;
use crate::io::{Decoder, DecodingError, ListHeader, WireType};
use crate::Packet;
use enc::DecodeFromReadPrefix;
use std::io::Read;

impl Decoder {
    //! Generic: List

    /// Decodes a generic list value from the `Read` prefix with the `first` byte.
    pub fn decode_packet_slice<P, R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Vec<P>, DecodingError>
    where
        P: Packet,
        R: Read,
    {
        match wire {
            WireType::List => {
                let header: ListHeader =
                    ListHeader::decode_from_read_prefix_with_first_byte(r, first)
                        .map_err(|e| DecodingError::from_list_header(e))?;
                if header.wire_type() != P::wire_type() {
                    Err(InvalidListWire(header.wire_type()))
                } else {
                    self.decode_list_value(header, r, |r, first| {
                        P::decode_from_read_prefix_with_first_byte(r, first)
                            .map_err(|e| DecodingError::PacketDecoding(e))
                    })
                }
            }
            _ => Err(DecodingError::InvalidWireType(wire)),
        }
    }
}
