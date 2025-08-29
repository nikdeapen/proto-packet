use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::{Message, Packet};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A message with special type slices.
/// message SpecialTypes {
///    
///    // A `[]uuid` field.
///    one: []uuid = 1;
///    
///    // A `[]string` field.
///    two: []string = 2;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct SpecialTypes {
    packet_unrecognized_fields: Vec<u8>,
    one: Option<Vec<uuid::Uuid>>,
    two: Option<Vec<String>>,
}

impl SpecialTypes {
    //! Field: `one`
    //!
    //! // A `[]uuid` field.
    //! one: []uuid = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<&[uuid::Uuid]> {
        self.one.as_deref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Option<Vec<uuid::Uuid>>
    where
        T: Into<Option<Vec<uuid::Uuid>>>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Option<Vec<uuid::Uuid>>>,
    {
        self.set_one(one);
        self
    }
}

impl SpecialTypes {
    //! Field: `two`
    //!
    //! // A `[]string` field.
    //! two: []string = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<&[String]> {
        self.two.as_deref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> Option<Vec<String>>
    where
        T: Into<Option<Vec<String>>>,
    {
        std::mem::replace(&mut self.two, two.into())
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<Option<Vec<String>>>,
    {
        self.set_two(two);
        self
    }
}

impl Packet for SpecialTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for SpecialTypes {}

impl EncodedLen for SpecialTypes {
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
                let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encoded_len()?
            };
        }

        if let Some(value) = &self.two {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::List, tag_number);
                header.encoded_len()?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<Vec<String>> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encoded_len()?
            };
        }

        Ok(encoded_len)
    }
}

impl EncodeToSlice for SpecialTypes {
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
                let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
            };
        }

        if let Some(value) = &self.two {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::List, tag_number);
                header.encode_to_slice_unchecked(&mut target[encoded_len..])?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<Vec<String>> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
            };
        }

        Ok(encoded_len)
    }
}

impl EncodeToWrite for SpecialTypes {
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
                let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encode_to_write(w)?
            };
        }

        if let Some(value) = &self.two {
            encoded_len += {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(WireType::List, tag_number);
                header.encode_to_write(w)?
            };
            encoded_len += {
                let encoder: proto_packet::io::Encoder<Vec<String>> =
                    proto_packet::io::Encoder::new(value, false);
                encoder.encode_to_write(w)?
            };
        }

        Ok(encoded_len)
    }
}

impl DecodeFromRead for SpecialTypes {
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
                    let value: Vec<uuid::Uuid> = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_uuid_slice(WireType::List, r, first)?
                    };
                    result.set_one(value);
                }
                2 => {
                    let value: Vec<String> = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_string_slice(WireType::List, r, first)?
                    };
                    result.set_two(value);
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

impl DecodeFromReadPrefix for SpecialTypes {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, StreamError>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
