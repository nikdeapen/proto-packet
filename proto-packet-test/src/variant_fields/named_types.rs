use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::{FieldHeader, WireType};
use proto_packet::{Packet, Variant};
use std::io::{Error, Read, Write};

/// A variant with named types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NamedTypes {
    /// // A local variant case.
    /// LocalVariant = 1;
    LocalVariant(crate::variant_fields::UnsignedInts),
}

impl NamedTypes {
    //! Tag Number

    /// Gets the tag number.
    pub fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::LocalVariant(_) => 1,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl Packet for NamedTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Variant for NamedTypes {}

impl EncodedLen for NamedTypes {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        match self {
            Self::LocalVariant(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::PacketField::from_packet(tag_number, value).encoded_len()
            }
        }
    }
}

impl EncodeToSlice for NamedTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        match self {
            Self::LocalVariant(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::PacketField::from_packet(tag_number, value)
                    .encode_to_slice_unchecked(target)
            }
        }
    }
}

impl EncodeToWrite for NamedTypes {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        match self {
            Self::LocalVariant(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::PacketField::from_packet(tag_number, value).encode_to_write(w)
            }
        }
    }
}

impl DecodeFromRead for NamedTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let field_header: FieldHeader = FieldHeader::decode_from_read_prefix(r)?;
        match field_header.tag_number().tag_number() {
            1 => {
                let value: crate::variant_fields::UnsignedInts =
                    proto_packet::io::decode_packet(field_header.wire_type(), r)?;
                Ok(Self::LocalVariant(value))
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

impl DecodeFromReadPrefix for NamedTypes {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use enc::DecodeFromRead;
        Self::decode_from_read_length_prefixed_with_first_byte(first, r)
    }
}
