use enc::var_int::VarInt32;
use enc::Error::IntegerOverflow;
use enc::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Write};

use crate::io::{TagNumber, WireType};

/// The header of a field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FieldHeader {
    wire_type: WireType,
    tag_number: TagNumber,
}

impl FieldHeader {
    //! Constants

    /// The maximum length of an encoded field header. (1 + 5 = 6)
    pub const MAX_ENCODED_LEN: usize = 1 + VarInt32::MAX_ENCODED_LEN;

    /// The maximum tag number for a field header encoded within a single byte. (31)
    pub const MAX_SINGLE_BYTE_TAG_NUMBER: u32 = 0x1F;
}

impl FieldHeader {
    //! Construction

    /// Creates a new `FieldHeader`.
    #[inline(always)]
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
}

impl EncodedLen for FieldHeader {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let len: usize = if self.tag_number.tag_number() <= Self::MAX_SINGLE_BYTE_TAG_NUMBER {
            1
        } else {
            let overflow: u32 = self.tag_number.tag_number() - Self::MAX_SINGLE_BYTE_TAG_NUMBER - 1;
            let overflow: usize = VarInt32::from(overflow).encoded_len()?;
            1 + overflow
        };
        Ok(len)
    }
}

impl EncodeToSlice for FieldHeader {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let len: usize = if self.tag_number.tag_number() <= Self::MAX_SINGLE_BYTE_TAG_NUMBER {
            let b: u8 = self.wire_type.to_high_3_bits() | (self.tag_number.tag_number() as u8);
            *target.get_unchecked_mut(0) = b;
            1
        } else {
            let first: u8 = self.wire_type.to_high_3_bits();
            *target.get_unchecked_mut(0) = first;

            let overflow: usize =
                VarInt32::from(self.tag_number.tag_number() - Self::MAX_SINGLE_BYTE_TAG_NUMBER - 1)
                    .encode_to_slice_unchecked(&mut target[1..])?;

            1 + overflow
        };
        Ok(len)
    }
}

impl EncodeToWrite for FieldHeader {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut buffer: [u8; Self::MAX_ENCODED_LEN] = [0u8; Self::MAX_ENCODED_LEN];
        let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer)? };
        w.write_all(&buffer[..encoded_len])?;
        Ok(encoded_len)
    }
}

impl DecodeFromReadPrefix for FieldHeader {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let wire_type: WireType = WireType::from_high_3_bits(first);
        let first: u8 = first & 0x1F;
        if first == 0 {
            let extra: u32 = VarInt32::decode_from_read_prefix(r)?.value;
            let tag_number: u32 = (Self::MAX_SINGLE_BYTE_TAG_NUMBER + 1)
                .checked_add(extra)
                .ok_or(IntegerOverflow)?;
            if let Some(tag_number) = TagNumber::new(tag_number) {
                Ok(Self {
                    wire_type,
                    tag_number,
                })
            } else {
                Err(Error::new(InvalidData, "invalid tag number"))
            }
        } else {
            Ok(Self {
                wire_type,
                tag_number: unsafe { TagNumber::new_unchecked(first as u32) },
            })
        }
    }
}
