use crate::io::{TagNumber, WireType};
use enc::var_int::{VarInt32, VarIntSize};
use enc::{impl_encode_to_write_stack_buf, DecodeFromReadPrefix, EncodeToSlice, EncodedLen, Error};
use std::io::Read;

/// A field header.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FieldHeader {
    wire_type: WireType,
    tag_number: TagNumber,
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
    pub const fn new(wire_type: WireType, tag_number: TagNumber) -> Self {
        Self {
            wire_type,
            tag_number,
        }
    }
}

impl FieldHeader {
    //! Properties

    /// Gets the wire type.
    pub fn wire_type(&self) -> WireType {
        self.wire_type
    }

    /// Gets the tag number.
    pub fn tag_number(&self) -> TagNumber {
        self.tag_number
    }

    /// Gets the `extra` portion of the tag number that doesn't fit in the first byte.
    ///
    /// # Safety
    /// The `tag_number` must be greater than `MAX_SINGLE_BYTE_TAG_NUMBER`.
    #[inline(always)]
    unsafe fn extra(&self) -> u32 {
        debug_assert!(self.tag_number.value() > Self::MAX_SINGLE_BYTE_TAG_NUMBER);

        self.tag_number.value() - Self::MAX_SINGLE_BYTE_TAG_NUMBER - 1
    }
}

impl EncodedLen for FieldHeader {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(
            if self.tag_number.value() <= Self::MAX_SINGLE_BYTE_TAG_NUMBER {
                1
            } else {
                1 + VarInt32::from(unsafe { self.extra() }).encoded_len()?
            },
        )
    }
}

impl EncodeToSlice for FieldHeader {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        Ok(
            if self.tag_number.value() <= Self::MAX_SINGLE_BYTE_TAG_NUMBER {
                let first: u8 = self.wire_type.to_high_3_bits() | (self.tag_number.value() as u8);
                *target.get_unchecked_mut(0) = first;
                1
            } else {
                *target.get_unchecked_mut(0) = self.wire_type.to_high_3_bits();
                let extra: usize = VarInt32::from(unsafe { self.extra() })
                    .encode_to_slice_unchecked(&mut target[1..])?;
                1 + extra
            },
        )
    }
}

impl_encode_to_write_stack_buf!(FieldHeader, Self::MAX_ENCODED_LEN);

impl DecodeFromReadPrefix for FieldHeader {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        let wire_type: WireType = WireType::from_high_3_bits(first);
        let tag_number: u32 = (first & 0x1F) as u32;
        let tag_number: u32 = if tag_number == 0 {
            let extra: u32 = VarInt32::decode_from_read_prefix(r)?.value();
            extra + 1 + Self::MAX_SINGLE_BYTE_TAG_NUMBER
        } else {
            tag_number
        };
        let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(tag_number) };
        Ok(Self::new(wire_type, tag_number))
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
            enc::test::test_encode(&header, *expected);
            enc::test::test_decode_from_read_prefix(*expected, &header, false);
        }
    }
}
