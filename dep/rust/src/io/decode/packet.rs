use crate::io::DecodingError::{InvalidWireType, PacketDecoding};
use crate::io::WireType::LengthPrefixed;
use crate::io::{Decoder, DecodingError, WireType};
use crate::Packet;
use enc::var_int::VarIntSize;
use enc::DecodeFromReadPrefix;
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
        if wire == P::wire_type() {
            if wire == LengthPrefixed {
                let prefix: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
                    .map_err(DecodingError::from_var_int_error)?
                    .value();
                let mut r: Take<&mut R> = r.take(prefix as u64); // todo cast
                P::decode_from_read(&mut r).map_err(PacketDecoding)
            } else {
                Ok(P::decode_from_read_prefix_with_first_byte(r, first).map_err(PacketDecoding)?)
            }
        } else {
            Err(InvalidWireType(wire))
        }
    }
}
