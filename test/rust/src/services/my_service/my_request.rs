use enc::Error;
use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Message, Packet, PacketType};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // My service request.
/// message MyRequest {
///    
///    message: string = 1;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct MyRequest {
    packet_unrecognized_fields: Vec<u8>,
    message: Option<String>,
}

impl MyRequest {
    //! Field: `message`
    //!
    //! message: string = 1;

    /// Gets the field: `message`.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Sets the field: `message`. Returns the previous value.
    pub fn set_message<T>(&mut self, message: T) -> Option<String>
    where
        T: Into<Option<String>>,
    {
        std::mem::replace(&mut self.message, message.into())
    }

    /// Sets the field: `message`. Returns the struct itself.
    pub fn with_message<T>(mut self, message: T) -> Self
    where
        T: Into<Option<String>>,
    {
        self.set_message(message);
        self
    }
}

impl Packet for MyRequest {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }

    fn packet_type() -> PacketType {
        PacketType::Message
    }
}

impl Message for MyRequest {}

impl EncodedLen for MyRequest {
    fn encoded_len(&self) -> Result<usize, Error> {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encoded_len!(
            &self.message,
            false,
            1,
            WireType::LengthPrefixed,
            encoded_len
        );

        Ok(encoded_len)
    }
}

impl EncodeToSlice for MyRequest {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.message,
            false,
            1,
            WireType::LengthPrefixed,
            encoded_len,
            &mut target[encoded_len..]
        );

        Ok(encoded_len)
    }
}

impl EncodeToWrite for MyRequest {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encode_to_write!(
            &self.message,
            false,
            1,
            WireType::LengthPrefixed,
            encoded_len,
            w
        );

        Ok(encoded_len)
    }
}

impl DecodeFromRead for MyRequest {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut result: Self = Self::default();

        while let Some(first) = enc::read_optional_byte(r)? {
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::decode_from_read_prefix_with_first_byte(r, first)?;
            match header.tag_number().value() {
                1 => {
                    let value: String = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_string(WireType::LengthPrefixed, r, first)?
                    };
                    result.set_message(value);
                }
                _ => {
                    let mut w: std::io::Cursor<&mut Vec<u8>> =
                        std::io::Cursor::new(&mut result.packet_unrecognized_fields);
                    header.encode_to_write(&mut w)?;
                    header.wire_type().transfer(r, &mut w)?;
                }
            }
        }
        Ok(result)
    }
}

impl DecodeFromReadPrefix for MyRequest {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
