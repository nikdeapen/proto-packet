use std::io::{Error, Write};

use enc::var_int::VarInt16;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::message::FieldHeader;
use crate::io::WireType;

/// A `PrimitiveType::UnsignedInt16` field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct UnsignedInt16Field {
    field_number: u32,
    fixed: bool,
    value: Option<u16>,
}

impl UnsignedInt16Field {
    //! Construction

    /// Creates a new `PrimitiveType::UnsignedInt16` field.
    #[inline(always)]
    pub fn new(field_number: u32, fixed: bool, value: Option<u16>) -> Self {
        debug_assert!(field_number != 0);

        Self {
            field_number,
            fixed,
            value,
        }
    }
}

impl UnsignedInt16Field {
    //! Constants

    /// The maximum encoded length of a `PrimitiveType::UnsignedInt16` field.
    pub const MAX_ENCODED_LEN: usize = FieldHeader::MAX_ENCODED_LEN + VarInt16::MAX_ENCODED_LEN;
}

impl UnsignedInt16Field {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        let wire_type: WireType = if self.fixed {
            WireType::Fixed2Bytes
        } else {
            WireType::VarInt
        };
        FieldHeader::new(wire_type, self.field_number)
    }
}

impl EncodedLen for UnsignedInt16Field {
    fn encoded_len(&self) -> usize {
        if let Some(value) = self.value {
            let h: usize = self.field_header().encoded_len();
            let v: usize = if self.fixed {
                2
            } else {
                VarInt16::from(value).encoded_len()
            };
            h + v
        } else {
            0
        }
    }
}

impl EncodeToSlice for UnsignedInt16Field {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> usize {
        if let Some(value) = self.value {
            let h: usize = self.field_header().encode_to_slice_unchecked(target);
            let v: usize = if self.fixed {
                let mut value: [u8; 2] = value.to_le_bytes();
                let target: &mut [u8] = &mut target[h..];

                *target.get_unchecked_mut(0) = *value.get_unchecked_mut(0);
                *target.get_unchecked_mut(1) = *value.get_unchecked_mut(1);

                2
            } else {
                VarInt16::from(value).encode_to_slice_unchecked(&mut target[h..])
            };
            h + v
        } else {
            0
        }
    }
}

impl EncodeToWrite for UnsignedInt16Field {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut buffer: [u8; Self::MAX_ENCODED_LEN] = [0u8; Self::MAX_ENCODED_LEN];
        let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer) };
        w.write_all(&mut buffer[..encoded_len])?;
        Ok(encoded_len)
    }
}
