use std::io::{Error, Read, Write};

use enc::var_int::VarIntSize;
use enc::Error::IntegerOverflow;
use enc::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::WireType;

/// The header of a `WireType::List`.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ListHeader {
    wire_type: WireType,
    list_size_bytes: usize,
}

impl ListHeader {
    //! Constants

    /// The maximum length of an encoded list header.
    pub const MAX_ENCODED_LEN: usize = 1 + VarIntSize::MAX_ENCODED_LEN;

    /// The maximum list size bytes for a list header encoded within a single byte. (31)
    pub const MAX_SINGLE_BYTE_LIST_SIZE_BYTES: usize = 0x1F;
}

impl ListHeader {
    //! Construction

    /// Creates a new `ListHeader`.
    #[inline(always)]
    pub const fn new(wire_type: WireType, list_size_bytes: usize) -> Self {
        Self {
            wire_type,
            list_size_bytes,
        }
    }
}

impl ListHeader {
    //! Properties

    /// Gets the wire type.
    pub fn wire_type(&self) -> WireType {
        self.wire_type
    }

    /// Gets the list size.
    pub fn list_size_bytes(&self) -> usize {
        self.list_size_bytes
    }
}

impl EncodedLen for ListHeader {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let len: usize = if self.list_size_bytes <= Self::MAX_SINGLE_BYTE_LIST_SIZE_BYTES {
            1
        } else {
            let overflow: usize = self.list_size_bytes - Self::MAX_SINGLE_BYTE_LIST_SIZE_BYTES - 1;
            let overflow: usize = VarIntSize::from(overflow).encoded_len()?;
            1 + overflow
        };
        Ok(len)
    }
}

impl EncodeToSlice for ListHeader {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let len: usize = if self.list_size_bytes <= Self::MAX_SINGLE_BYTE_LIST_SIZE_BYTES {
            let b: u8 = self.wire_type.to_high_3_bits() | (self.list_size_bytes as u8);
            *target.get_unchecked_mut(0) = b;
            1
        } else {
            let first: u8 = self.wire_type.to_high_3_bits();
            *target.get_unchecked_mut(0) = first;

            let overflow: usize =
                VarIntSize::from(self.list_size_bytes - Self::MAX_SINGLE_BYTE_LIST_SIZE_BYTES - 1)
                    .encode_to_slice_unchecked(&mut target[1..])?;

            1 + overflow
        };
        Ok(len)
    }
}

impl EncodeToWrite for ListHeader {
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

impl DecodeFromReadPrefix for ListHeader {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let wire_type: WireType = WireType::from_high_3_bits(first);
        let first: u8 = first & 0x1F;
        Ok(if first == 0 {
            let extra: usize = VarIntSize::decode_from_read_prefix(r)?.value;
            let list_size_bytes: usize = (Self::MAX_SINGLE_BYTE_LIST_SIZE_BYTES + 1)
                .checked_add(extra)
                .ok_or(IntegerOverflow)?;
            Self::new(wire_type, list_size_bytes)
        } else {
            Self {
                wire_type,
                list_size_bytes: first as usize,
            }
        })
    }
}
