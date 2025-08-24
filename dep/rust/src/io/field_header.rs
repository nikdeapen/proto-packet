use crate::io::{TagNumber, WireType};
use enc::var_int::{VarInt32, VarIntSize};
use enc::{
    read_optional_byte, DecodeFromRead, DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite,
    EncodedLen, Error, StreamError,
};
use std::io::{Read, Write};

/// A field header.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FieldHeader {
    wire: WireType,
    tag: TagNumber,
}

impl FieldHeader {
    //! Constants

    /// The maximum tag number for a field header encoded within a single byte. (31)
    pub const MAX_SINGLE_BYTE_TAG_NUMBER: u32 = 0x1F;

    /// The maximum encoded length of a field header.
    pub const MAX_ENCODED_LEN: usize = 1 + VarIntSize::MAX_ENCODED_LEN;
}

impl FieldHeader {
    //! Construction

    /// Creates a new field header.
    pub const fn new(wire: WireType, tag: TagNumber) -> Self {
        Self { wire, tag }
    }
}

impl FieldHeader {
    //! Properties

    /// Gets the wire type.
    pub fn wire_type(&self) -> WireType {
        self.wire
    }

    /// Gets the tag number.
    pub fn tag_number(&self) -> TagNumber {
        self.tag
    }
}

impl EncodedLen for FieldHeader {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(if self.tag.value() <= Self::MAX_SINGLE_BYTE_TAG_NUMBER {
            1
        } else {
            let extra: u32 = self.tag.value() - Self::MAX_SINGLE_BYTE_TAG_NUMBER - 1;
            let extra: usize = VarInt32::from(extra).encoded_len()?;
            1 + extra
        })
    }
}

impl EncodeToSlice for FieldHeader {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        Ok(if self.tag.value() <= Self::MAX_SINGLE_BYTE_TAG_NUMBER {
            let first: u8 = self.wire.to_high_3_bits() | (self.tag.value() as u8);
            *target.get_unchecked_mut(0) = first;
            1
        } else {
            *target.get_unchecked_mut(0) = self.wire.to_high_3_bits();

            let extra: u32 = self.tag.value() - Self::MAX_SINGLE_BYTE_TAG_NUMBER - 1;
            let extra: usize = VarInt32::from(extra).encode_to_slice_unchecked(&mut target[1..])?;

            1 + extra
        })
    }
}

impl EncodeToWrite for FieldHeader {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: Write,
    {
        let mut buffer: [u8; Self::MAX_ENCODED_LEN] = [0u8; Self::MAX_ENCODED_LEN];
        let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer)? };
        w.write_all(&buffer[..encoded_len])?;
        Ok(encoded_len)
    }
}

impl DecodeFromRead for FieldHeader {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let header: FieldHeader = FieldHeader::decode_from_read_prefix(r)?;
        debug_assert_eq!(read_optional_byte(r)?, None);
        Ok(header)
    }
}

impl DecodeFromReadPrefix for FieldHeader {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let wire: WireType = WireType::from_high_3_bits(first);
        let tag: u32 = (first & 0x1F) as u32;
        let tag: u32 = if tag == 0 {
            let extra: u32 = VarInt32::decode_from_read_prefix(r)?.value();
            extra + 1 + Self::MAX_SINGLE_BYTE_TAG_NUMBER
        } else {
            tag
        };
        let tag: TagNumber = unsafe { TagNumber::new_unchecked(tag) };
        Ok(Self::new(wire, tag))
    }
}

#[cfg(test)]
mod test {
    use crate::io::WireType::*;
    use crate::io::{FieldHeader, TagNumber, WireType};

    #[test]
    fn io() {
        let test_cases: &[(WireType, u32, &[u8])] = &[
            (Fixed1Byte, 1, &[0b0000_0001]),
            (Fixed2Byte, 2, &[0b0010_0010]),
            (Fixed4Byte, 3, &[0b0100_0011]),
            (Fixed8Byte, 0x1F, &[0b0111_1111]),
            (Fixed16Byte, 0x20, &[0b1000_0000, 0]),
            (VarInt, 0x21, &[0b1010_0000, 1]),
            (LengthPrefixed, 0x22, &[0b1100_0000, 2]),
            (
                List,
                TagNumber::MAX_TAG_NUMBER,
                &[0b1110_0000, 223, 255, 255, 255, 7],
            ),
        ];
        for (wire, tag, expected) in test_cases {
            let header: FieldHeader = FieldHeader::new(*wire, TagNumber::new(*tag).unwrap());
            enc::test::test_io(&header, *expected, false);
        }
    }
}
