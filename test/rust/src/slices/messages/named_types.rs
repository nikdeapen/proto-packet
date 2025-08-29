use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::{Message, Packet};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A message with named type slices.
/// message NamedTypes {
///    
///    // A `[]message` field.
///    one: []slices.messages.PrimitiveTypes = 1;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct NamedTypes {
    packet_unrecognized_fields: Vec<u8>,
    one: Option<Vec<crate::slices::messages::PrimitiveTypes>>,
}

impl NamedTypes {
    //! Field: `one`
    //!
    //! // A `[]message` field.
    //! one: []slices.messages.PrimitiveTypes = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<&[crate::slices::messages::PrimitiveTypes]> {
        self.one.as_deref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Option<Vec<crate::slices::messages::PrimitiveTypes>>
    where
        T: Into<Option<Vec<crate::slices::messages::PrimitiveTypes>>>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Option<Vec<crate::slices::messages::PrimitiveTypes>>>,
    {
        self.set_one(one);
        self
    }
}

impl Packet for NamedTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for NamedTypes {}

impl EncodedLen for NamedTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::List, tag_number);
                header.encoded_len()?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<
                    Vec<crate::slices::messages::PrimitiveTypes>,
                > = proto_packet::io::Encoder::new(value, false);
                encoder.encoded_len()?
            };
        }

        Ok(encoded_len)
    }
}

impl EncodeToSlice for NamedTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::List, tag_number);
                header.encode_to_slice_unchecked(&mut target[encoded_len..])?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<
                    Vec<crate::slices::messages::PrimitiveTypes>,
                > = proto_packet::io::Encoder::new(value, false);
                encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
            };
        }

        Ok(encoded_len)
    }
}

impl EncodeToWrite for NamedTypes {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::List, tag_number);
                header.encode_to_write(w)?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<
                    Vec<crate::slices::messages::PrimitiveTypes>,
                > = proto_packet::io::Encoder::new(value, false);
                encoder.encode_to_write(w)?
            };
        }

        Ok(encoded_len)
    }
}

impl DecodeFromRead for NamedTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let mut result: Self = Self::default();

        while let Some(first) = enc::read_optional_byte(r)? {
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::decode_from_read_prefix_with_first_byte(r, first)?;
            match header.tag_number().value() {
                1 => {
                    let value: Vec<crate::slices::messages::PrimitiveTypes> = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_packet_slice(WireType::List, r, first)?
                    };
                    result.set_one(value);
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

impl DecodeFromReadPrefix for NamedTypes {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, StreamError>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
