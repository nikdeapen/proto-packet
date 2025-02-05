use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Enum, Packet};
use std::io::{Error, Read, Write};
use std::str::FromStr;

/// An empty enum.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum EmptyEnum {
    /// An enum case with an unrecognized tag number.
    Unrecognized(proto_packet::io::TagNumber),
}

impl FromStr for EmptyEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => Err(()),
        }
    }
}

impl EmptyEnum {
    //! Tag Numbers

    /// Gets the tag number.
    pub fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::Unrecognized(tag_number) => tag_number.tag_number(),
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl From<proto_packet::io::TagNumber> for EmptyEnum {
    fn from(tag_number: proto_packet::io::TagNumber) -> Self {
        match tag_number.tag_number() {
            _ => Self::Unrecognized(tag_number),
        }
    }
}

impl Packet for EmptyEnum {
    fn wire_type() -> WireType {
        WireType::VarInt
    }
}

impl Enum for EmptyEnum {}

impl EncodedLen for EmptyEnum {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        enc::var_int::VarInt32::from(self.tag_number().tag_number()).encoded_len()
    }
}

impl EncodeToSlice for EmptyEnum {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        enc::var_int::VarInt32::from(self.tag_number().tag_number())
            .encode_to_slice_unchecked(target)
    }
}

impl EncodeToWrite for EmptyEnum {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        enc::var_int::VarInt32::from(self.tag_number().tag_number()).encode_to_write(w)
    }
}

impl DecodeFromRead for EmptyEnum {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use enc::DecodeFromReadPrefix;
        Self::decode_from_read_prefix(r)
    }
}

impl DecodeFromReadPrefix for EmptyEnum {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let tag_number: u32 =
            enc::var_int::VarInt32::decode_from_read_prefix_with_first_byte(first, r)?.value;
        if let Some(tag_number) = proto_packet::io::TagNumber::new(tag_number) {
            Ok(Self::from(tag_number))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid tag number",
            ))
        }
    }
}
