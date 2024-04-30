use std::io::{Error, Write};

use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::message::FieldHeader;
use crate::io::WireType;

/// A `PrimitiveType::UnsignedInt8` field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct UnsignedInt8Field {
    field_number: u32,
    value: Option<u8>,
}

impl UnsignedInt8Field {
    //! Construction

    /// Creates a new `PrimitiveType::UnsignedInt8` field.
    #[inline(always)]
    pub fn new(field_number: u32, value: Option<u8>) -> Self {
        debug_assert!(field_number != 0);

        Self {
            field_number,
            value,
        }
    }
}

impl UnsignedInt8Field {
    //! Constants

    /// The maximum encoded length of a `PrimitiveType::UnsignedInt8` field.
    pub const MAX_ENCODED_LEN: usize = FieldHeader::MAX_ENCODED_LEN + 1;
}

impl UnsignedInt8Field {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(WireType::Fixed1Byte, self.field_number)
    }
}

impl EncodedLen for UnsignedInt8Field {
    fn encoded_len(&self) -> usize {
        if self.value.is_some() {
            self.field_header().encoded_len() + 1
        } else {
            0
        }
    }
}

impl EncodeToSlice for UnsignedInt8Field {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> usize {
        if let Some(value) = self.value {
            let header_len: usize = self.field_header().encode_to_slice_unchecked(target);
            *target.get_unchecked_mut(header_len) = value;
            header_len + 1
        } else {
            0
        }
    }
}

impl EncodeToWrite for UnsignedInt8Field {
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
