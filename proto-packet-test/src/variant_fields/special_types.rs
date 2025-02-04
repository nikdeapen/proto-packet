use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::{FieldHeader, WireType};
use proto_packet::{Packet, Variant};
use std::io::{Error, Read, Write};

/// A variant with special types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SpecialTypes {
    /// // The first case.
    /// One = 1;
    One(uuid::Uuid),

    /// // The second case.
    /// Two = 2;
    Two(String),
}

impl SpecialTypes {
    //! Tag Number

    /// Gets the tag number.
    pub fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::One(_) => 1,
            Self::Two(_) => 2,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl Packet for SpecialTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Variant for SpecialTypes {}

impl EncodedLen for SpecialTypes {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        match self {
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::Fixed16ByteField::from_uuid(tag_number, value).encoded_len()
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                proto_packet::io::BytesField::from_string(tag_number, value).encoded_len()
            }
        }
    }
}

impl EncodeToSlice for SpecialTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        match self {
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::Fixed16ByteField::from_uuid(tag_number, value)
                    .encode_to_slice_unchecked(target)
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                proto_packet::io::BytesField::from_string(tag_number, value)
                    .encode_to_slice_unchecked(target)
            }
        }
    }
}

impl EncodeToWrite for SpecialTypes {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        match self {
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::Fixed16ByteField::from_uuid(tag_number, value).encode_to_write(w)
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                proto_packet::io::BytesField::from_string(tag_number, value).encode_to_write(w)
            }
        }
    }
}

impl DecodeFromRead for SpecialTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let field_header: FieldHeader = FieldHeader::decode_from_read_prefix(r)?;
        match field_header.tag_number().tag_number() {
            1 => {
                let value: uuid::Uuid = proto_packet::io::decode_uuid(field_header.wire_type(), r)?;
                Ok(Self::One(value))
            }
            2 => {
                let value: String = proto_packet::io::decode_string(field_header.wire_type(), r)?;
                Ok(Self::Two(value))
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

impl DecodeFromReadPrefix for SpecialTypes {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use enc::DecodeFromRead;
        Self::decode_from_read_length_prefixed_with_first_byte(first, r)
    }
}
