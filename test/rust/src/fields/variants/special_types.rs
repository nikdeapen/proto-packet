use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::io::WithTagNumber;
use proto_packet::{Packet, Variant};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// A variant with special type cases.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum SpecialTypes {
    /// // A `uuid` case.
    /// One: uuid = 1;
    One(uuid::Uuid),

    /// // A `string` case.
    /// Two: string = 2;
    Two(String),
}

impl Packet for SpecialTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Variant for SpecialTypes {}

impl WithTagNumber for SpecialTypes {
    fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::One(_) => 1,
            Self::Two(_) => 2,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl EncodedLen for SpecialTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        match self {
            Self::One(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::Fixed16Byte, tag_number);
                    header.encoded_len()?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encoded_len()?
                };
            }
            Self::Two(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
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
        }
        Ok(encoded_len)
    }
}

impl EncodeToSlice for SpecialTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        match self {
            Self::One(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::Fixed16Byte, tag_number);
                    header.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
            }
            Self::Two(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
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
        match self {
            Self::One(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::Fixed16Byte, tag_number);
                    header.encode_to_write(w)?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_write(w)?
                };
            }
            Self::Two(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
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
        }
        Ok(encoded_len)
    }
}

impl DecodeFromRead for SpecialTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let first: u8 = enc::read_single_byte(r)?;
        let header: proto_packet::io::FieldHeader =
            proto_packet::io::FieldHeader::decode_from_read_prefix_with_first_byte(r, first)?;
        match header.tag_number().value() {
            1 => {
                let value: uuid::Uuid = {
                    let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_uuid(WireType::Fixed16Byte, r, first)?
                };
                Ok(Self::One(value))
            }
            2 => {
                let value: String = {
                    let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_string(WireType::LengthPrefixed, r, first)?
                };
                Ok(Self::Two(value))
            }
            _ => {
                todo!()
            }
        }
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
