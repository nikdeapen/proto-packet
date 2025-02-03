use std::io::{Error, Write};

use enc::var_int::VarInt64;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::{FieldHeader, TagNumber, WireType};

/// A max 64-bit `VarInt` field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VarInt64Field {
    tag_number: TagNumber,
    value: u64,
}

impl VarInt64Field {
    //! Constants

    /// The maximum encoded length of a `VarInt64Field`. (6 + 10 = 16)
    pub const MAX_ENCODED_LEN: usize = FieldHeader::MAX_ENCODED_LEN + VarInt64::MAX_ENCODED_LEN;
}

impl VarInt64Field {
    //! Construction

    /// Creates a new `VarInt64Field`.
    #[inline(always)]
    pub const fn new(tag_number: TagNumber, value: u64) -> Self {
        Self { tag_number, value }
    }

    /// Creates a new `VarInt64Field`.
    #[inline(always)]
    pub const fn from_u64(tag_number: TagNumber, value: &u64) -> Self {
        Self::new(tag_number, *value)
    }
}

impl VarInt64Field {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(WireType::VarInt, self.tag_number)
    }
}

impl EncodedLen for VarInt64Field {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        Ok(self.field_header().encoded_len()? + VarInt64::from(self.value).encoded_len()?)
    }
}

impl EncodeToSlice for VarInt64Field {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let header_len: usize = self.field_header().encode_to_slice_unchecked(target)?;
        let value_len: usize =
            VarInt64::from(self.value).encode_to_slice_unchecked(&mut target[header_len..])?;
        Ok(header_len + value_len)
    }
}

impl EncodeToWrite for VarInt64Field {
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
