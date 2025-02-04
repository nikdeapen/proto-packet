use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::{FieldHeader, WireType};
use proto_packet::{Packet, Variant};
use std::io::{Error, Read, Write};

/// A variant with unsigned integers.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum UnsignedInts {
    /// // The first case.
    /// One = 1;
    One(u8),

    /// // The second case.
    /// Two = 2;
    Two(u16),

    /// // The third case.
    /// Three = 3;
    Three(u32),

    /// // The fourth case.
    /// Four = 4;
    Four(u64),

    /// // The fifth case.
    /// Five = 5;
    Five(u128),
}

impl UnsignedInts {
    //! Tag Number

    /// Gets the tag number.
    pub fn tag_number(&self) -> proto_packet::io::TagNumber {
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

impl Packet for UnsignedInts {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Variant for UnsignedInts {}

impl EncodedLen for UnsignedInts {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        match self {
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::Fixed1ByteField::from_u8(tag_number, value).encoded_len()
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                proto_packet::io::VarInt16Field::from_u16(tag_number, value).encoded_len()
            }
            Self::Three(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
                proto_packet::io::VarInt32Field::from_u32(tag_number, value).encoded_len()
            }
            Self::Four(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
                proto_packet::io::VarInt64Field::from_u64(tag_number, value).encoded_len()
            }
            Self::Five(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
                proto_packet::io::VarInt128Field::from_u128(tag_number, value).encoded_len()
            }
        }
    }
}

impl EncodeToSlice for UnsignedInts {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        match self {
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::Fixed1ByteField::from_u8(tag_number, value)
                    .encode_to_slice_unchecked(target)
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                proto_packet::io::VarInt16Field::from_u16(tag_number, value)
                    .encode_to_slice_unchecked(target)
            }
            Self::Three(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
                proto_packet::io::VarInt32Field::from_u32(tag_number, value)
                    .encode_to_slice_unchecked(target)
            }
            Self::Four(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
                proto_packet::io::VarInt64Field::from_u64(tag_number, value)
                    .encode_to_slice_unchecked(target)
            }
            Self::Five(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
                proto_packet::io::VarInt128Field::from_u128(tag_number, value)
                    .encode_to_slice_unchecked(target)
            }
        }
    }
}

impl EncodeToWrite for UnsignedInts {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        match self {
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                proto_packet::io::Fixed1ByteField::from_u8(tag_number, value).encode_to_write(w)
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                proto_packet::io::VarInt16Field::from_u16(tag_number, value).encode_to_write(w)
            }
            Self::Three(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
                proto_packet::io::VarInt32Field::from_u32(tag_number, value).encode_to_write(w)
            }
            Self::Four(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
                proto_packet::io::VarInt64Field::from_u64(tag_number, value).encode_to_write(w)
            }
            Self::Five(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
                proto_packet::io::VarInt128Field::from_u128(tag_number, value).encode_to_write(w)
            }
        }
    }
}

impl DecodeFromRead for UnsignedInts {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let field_header: FieldHeader = FieldHeader::decode_from_read_prefix(r)?;
        match field_header.tag_number().tag_number() {
            1 => {
                let value: u8 = proto_packet::io::decode_u8(field_header.wire_type(), r)?;
                Ok(Self::One(value))
            }
            2 => {
                let value: u16 = proto_packet::io::decode_u16(field_header.wire_type(), r)?;
                Ok(Self::Two(value))
            }
            3 => {
                let value: u32 = proto_packet::io::decode_u32(field_header.wire_type(), r)?;
                Ok(Self::Three(value))
            }
            4 => {
                let value: u64 = proto_packet::io::decode_u64(field_header.wire_type(), r)?;
                Ok(Self::Four(value))
            }
            5 => {
                let value: u128 = proto_packet::io::decode_u128(field_header.wire_type(), r)?;
                Ok(Self::Five(value))
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

impl DecodeFromReadPrefix for UnsignedInts {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use enc::DecodeFromRead;
        Self::decode_from_read_length_prefixed_with_first_byte(first, r)
    }
}
