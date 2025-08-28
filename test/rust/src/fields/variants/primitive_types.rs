use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::io::WithTagNumber;
use proto_packet::{Packet, Variant};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// A variant with primitive type cases.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum PrimitiveTypes {
    /// // A `u8` case.
    /// One: u8 = 1;
    One(u8),

    /// // A `u16` case.
    /// Two: u16 = 2;
    Two(u16),

    /// // A `u32` case.
    /// Three: u32 = 3;
    Three(u32),

    /// // A `u64` case.
    /// Four: u64 = 4;
    Four(u64),

    /// // A `u128` case.
    /// Five: u128 = 5;
    Five(u128),
}

impl Packet for PrimitiveTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Variant for PrimitiveTypes {}

impl WithTagNumber for PrimitiveTypes {
    fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::One(_) => 1,
            Self::Two(_) => 2,
            Self::Three(_) => 3,
            Self::Four(_) => 4,
            Self::Five(_) => 5,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl EncodedLen for PrimitiveTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        match self {
            Self::One(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::Fixed1Byte, tag_number);
                    header.encoded_len()?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u8> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encoded_len()?
                };
            }
            Self::Two(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encoded_len()?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u16> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encoded_len()?
                };
            }
            Self::Three(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encoded_len()?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u32> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encoded_len()?
                };
            }
            Self::Four(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encoded_len()?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u64> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encoded_len()?
                };
            }
            Self::Five(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encoded_len()?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u128> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encoded_len()?
                };
            }
        }
        Ok(encoded_len)
    }
}

impl EncodeToSlice for PrimitiveTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        match self {
            Self::One(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::Fixed1Byte, tag_number);
                    header.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u8> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
            }
            Self::Two(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u16> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
            }
            Self::Three(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u32> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
            }
            Self::Four(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u64> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
            }
            Self::Five(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u128> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
                };
            }
        }
        Ok(encoded_len)
    }
}

impl EncodeToWrite for PrimitiveTypes {
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
                        proto_packet::io::FieldHeader::new(WireType::Fixed1Byte, tag_number);
                    header.encode_to_write(w)?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u8> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_write(w)?
                };
            }
            Self::Two(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encode_to_write(w)?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u16> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_write(w)?
                };
            }
            Self::Three(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encode_to_write(w)?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u32> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_write(w)?
                };
            }
            Self::Four(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encode_to_write(w)?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u64> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_write(w)?
                };
            }
            Self::Five(value) => {
                encoded_len += {
                    let tag_number: proto_packet::io::TagNumber =
                        unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
                    let header: proto_packet::io::FieldHeader =
                        proto_packet::io::FieldHeader::new(WireType::VarInt, tag_number);
                    header.encode_to_write(w)?
                };
                encoded_len += {
                    let encoder: proto_packet::io::Encoder<u128> =
                        proto_packet::io::Encoder::new(value, false);
                    encoder.encode_to_write(w)?
                };
            }
        }
        Ok(encoded_len)
    }
}

impl DecodeFromRead for PrimitiveTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let first: u8 = enc::read_single_byte(r)?;
        let header: proto_packet::io::FieldHeader =
            proto_packet::io::FieldHeader::decode_from_read_prefix_with_first_byte(r, first)?;
        match header.tag_number().value() {
            1 => {
                let value: u8 = {
                    let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_u8(WireType::Fixed1Byte, r, first)?
                };
                Ok(Self::One(value))
            }
            2 => {
                let value: u16 = {
                    let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_u16(WireType::VarInt, r, first)?
                };
                Ok(Self::Two(value))
            }
            3 => {
                let value: u32 = {
                    let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_u32(WireType::VarInt, r, first)?
                };
                Ok(Self::Three(value))
            }
            4 => {
                let value: u64 = {
                    let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_u64(WireType::VarInt, r, first)?
                };
                Ok(Self::Four(value))
            }
            5 => {
                let value: u128 = {
                    let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_u128(WireType::VarInt, r, first)?
                };
                Ok(Self::Five(value))
            }
            _ => {
                todo!()
            }
        }
    }
}

impl DecodeFromReadPrefix for PrimitiveTypes {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, StreamError>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
