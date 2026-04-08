use crate::Packet;
use crate::io::DecodingError::InvalidWireType;
use crate::io::WireType::LengthPrefixed;
use crate::io::{Decoder, DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarIntSize;
use std::io::{Read, Take};

impl Decoder {
    //! Decode: `Packet`

    /// Decodes a `Packet` value from the `Read` prefix with the `first` byte.
    pub fn decode_packet<P, R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<P, DecodingError>
    where
        P: Packet,
        R: Read,
    {
        if wire == P::wire() {
            if wire == LengthPrefixed {
                let prefix: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
                    .map_err(DecodingError::from_length_prefix_error)?
                    .value();
                const _: () = assert!(usize::BITS <= 64);
                let mut r: Take<&mut R> = r.take(prefix as u64);
                let result: P =
                    P::decode_from_read(&mut r).map_err(DecodingError::from_packet_error)?;
                let unread: u64 = r.limit();
                if unread != 0 {
                    return Err(DecodingError::PacketUnderRead {
                        declared: prefix,
                        unread: unread as usize,
                    });
                }
                Ok(result)
            } else {
                P::decode_from_read_prefix_with_first_byte(r, first)
                    .map_err(DecodingError::from_packet_error)
            }
        } else {
            Err(InvalidWireType {
                semantic: "Packet",
                wire,
            })
        }
    }
}
