use crate::Packet;
use crate::io::DecodingError;
use crate::io::DecodingError::InvalidWireType;
use crate::io::WireType::List;
use crate::io::{Decoder, ListHeader, WireType};
use enc::{DecodeFromReadPrefix, read_single_byte};
use std::io::{Read, Take};

impl Decoder {
    //! Decode: `Vec<Packet>`

    /// Decodes a `Vec<Packet>` from the `Read` prefix with the `first` byte.
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
        if wire != List {
            return Err(InvalidWireType {
                semantic: "Vec<Packet>",
                wire,
            });
        }

        let header: ListHeader = ListHeader::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(DecodingError::from_length_prefix_error)?;

        if header.wire() != P::wire() {
            return Err(InvalidWireType {
                semantic: "Vec<Packet>",
                wire: header.wire(),
            });
        }

        const _: () = assert!(usize::BITS <= 64);
        let mut r: Take<&mut R> = r.take(header.size() as u64);
        let mut result: Vec<P> = Vec::with_capacity(header.element_capacity_hint());

        while r.limit() > 0 {
            let first: u8 = read_single_byte(&mut r).map_err(DecodingError::from)?;
            let packet: P = self.decode_packet(header.wire(), &mut r, first)?;
            result.push(packet);
        }

        Ok(result)
    }
}
