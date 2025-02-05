use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Enum, Packet};
use std::io::{Error, Read, Write};
use std::str::FromStr;

/// An enum with multiple cases.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum MultipleCases {
    /// An enum case with an unrecognized tag number.
    Unrecognized(proto_packet::io::TagNumber),

    /// // The first case.
    /// One = 1;
    One,

    /// // The second case.
    /// Two = 2;
    Two,

    /// // The third case.
    /// Three = 3;
    Three,
}

impl FromStr for MultipleCases {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "One" => Ok(Self::One),
            "Two" => Ok(Self::Two),
            "Three" => Ok(Self::Three),
            _ => Err(()),
        }
    }
}

impl MultipleCases {
    //! Tag Numbers

    /// Gets the tag number.
    pub fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Unrecognized(tag_number) => tag_number.tag_number(),
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl From<proto_packet::io::TagNumber> for MultipleCases {
    fn from(tag_number: proto_packet::io::TagNumber) -> Self {
        match tag_number.tag_number() {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            _ => Self::Unrecognized(tag_number),
        }
    }
}

impl Packet for MultipleCases {
    fn wire_type() -> WireType {
        WireType::VarInt
    }
}

impl Enum for MultipleCases {}

impl EncodedLen for MultipleCases {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        enc::var_int::VarInt32::from(self.tag_number().tag_number()).encoded_len()
    }
}

impl EncodeToSlice for MultipleCases {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        enc::var_int::VarInt32::from(self.tag_number().tag_number())
            .encode_to_slice_unchecked(target)
    }
}

impl EncodeToWrite for MultipleCases {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        enc::var_int::VarInt32::from(self.tag_number().tag_number()).encode_to_write(w)
    }
}

impl DecodeFromRead for MultipleCases {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use enc::DecodeFromReadPrefix;
        Self::decode_from_read_prefix(r)
    }
}

impl DecodeFromReadPrefix for MultipleCases {
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
