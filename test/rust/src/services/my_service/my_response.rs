use enc::Error;
use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Message, Packet};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // My service response.
/// message MyResponse {
///    
///    message: string = 1;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct MyResponse {
    packet_unrecognized_fields: Vec<u8>,
    message: Option<String>,
}

impl MyResponse {
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

impl Packet for MyResponse {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for MyResponse {}

impl EncodedLen for MyResponse {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.message {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::LengthPrefixed, tag_number);
                header.encoded_len()?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<String> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encoded_len()?
            };
        }

        Ok(encoded_len)
    }
}

impl EncodeToSlice for MyResponse {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.message {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::LengthPrefixed, tag_number);
                header.encode_to_slice_unchecked(&mut target[encoded_len..])?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<String> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
            };
        }

        Ok(encoded_len)
    }
}

impl EncodeToWrite for MyResponse {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.message {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::LengthPrefixed, tag_number);
                header.encode_to_write(w)?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<String> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encode_to_write(w)?
            };
        }

        Ok(encoded_len)
    }
}

impl DecodeFromRead for MyResponse {
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

impl DecodeFromReadPrefix for MyResponse {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
