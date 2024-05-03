use std::io::{Error, Write};

use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::message::FieldHeader;
use crate::io::WireType;

/// The maximum length of an encoded `BytesField` without the value.
const MAX_ENC_LEN_NO_VALUE: usize = FieldHeader::MAX_ENCODED_LEN + VarIntSize::MAX_ENCODED_LEN;

/// A field that can be represented as bytes.
///
/// # Applicable Fields:
/// - `SpecialType::String`
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct BytesField<'a> {
    field_number: u32,
    value: Option<&'a [u8]>,
}

impl<'a> BytesField<'a> {
    //! Construction

    /// Creates a new `SpecialType::String` field.
    #[inline(always)]
    pub fn new(field_number: u32, value: Option<&'a [u8]>) -> Self {
        debug_assert!(field_number != 0);

        Self {
            field_number,
            value,
        }
    }
}

impl<'a> BytesField<'a> {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(WireType::LengthPrefixed, self.field_number)
    }
}

impl<'a> EncodedLen for BytesField<'a> {
    fn encoded_len(&self) -> usize {
        if let Some(value) = self.value {
            self.field_header().encoded_len()
                + VarIntSize::from(value.len()).encoded_len()
                + value.len()
        } else {
            0
        }
    }
}

impl<'a> EncodeToSlice for BytesField<'a> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> usize {
        if let Some(value) = self.value {
            let header_len: usize = self.field_header().encode_to_slice_unchecked(target);
            let length_prefix_len: usize =
                VarIntSize::from(value.len()).encode_to_slice_unchecked(&mut target[header_len..]);

            let start: usize = header_len + length_prefix_len;
            let end: usize = start + value.len();
            (&mut target[start..end]).copy_from_slice(value);

            end
        } else {
            0
        }
    }
}

impl<'a> EncodeToWrite for BytesField<'a> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        if let Some(value) = self.value {
            let mut buffer: [u8; MAX_ENC_LEN_NO_VALUE] = [0u8; MAX_ENC_LEN_NO_VALUE];
            unsafe {
                let header_len: usize = self.field_header().encode_to_slice_unchecked(&mut buffer);
                let length_prefix_len: usize = VarIntSize::from(value.len())
                    .encode_to_slice_unchecked(&mut buffer[header_len..]);
                w.write_all(&buffer[..header_len + length_prefix_len])?;

                w.write_all(value)?;

                Ok(header_len + length_prefix_len + value.len())
            }
        } else {
            Ok(0)
        }
    }
}
