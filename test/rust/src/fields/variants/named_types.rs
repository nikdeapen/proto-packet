use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::io::WithTagNumber;
use proto_packet::{Packet, Variant};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// A variant with named type cases.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum NamedTypes {
    /// // A `variant` case.
    /// One: fields.variants.PrimitiveTypes = 1;
    One(crate::fields::variants::PrimitiveTypes),
}

impl Packet for NamedTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Variant for NamedTypes {}

impl WithTagNumber for NamedTypes {
    fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::One(_) => 1,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl EncodedLen for NamedTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        match self {
            Self::One(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                    let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                        crate::fields::variants::PrimitiveTypes::wire_type(),
                        tag_number,
                    );
                    header.encoded_len()?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<
                        crate::fields::variants::PrimitiveTypes,
                    > = proto_packet::io::Encoder::new(value, false);
                    encoder.encoded_len()?
                };
            }
        }
        Ok(encoded_len)
    }
}

impl EncodeToSlice for NamedTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        match self {
            Self::One(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                    let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                        crate::fields::variants::PrimitiveTypes::wire_type(),
                        tag_number,
                    );
                    header.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<
                        crate::fields::variants::PrimitiveTypes,
                    > = proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
            }
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
        match self {
            Self::One(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                    let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                        crate::fields::variants::PrimitiveTypes::wire_type(),
                        tag_number,
                    );
                    header.encode_to_write(w)?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<
                        crate::fields::variants::PrimitiveTypes,
                    > = proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_write(w)?
                };
            }
        }
        Ok(encoded_len)
    }
}

impl DecodeFromRead for NamedTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let first: u8 = enc::read_single_byte(r)?;
        let header: proto_packet::io::FieldHeader =
            proto_packet::io::FieldHeader::decode_from_read_prefix_with_first_byte(r, first)?;
        match header.tag_number().value() {
            1 => {
                let value: crate::fields::variants::PrimitiveTypes = {
                    let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_packet(
                        crate::fields::variants::PrimitiveTypes::wire_type(),
                        r,
                        first,
                    )?
                };
                Ok(Self::One(value))
            }
            _ => {
                todo!()
            }
        }
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
