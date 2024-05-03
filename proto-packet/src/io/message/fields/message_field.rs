use std::io::{Error, Write};

use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::message::FieldHeader;
use crate::io::WireType;
use crate::Message;

const MAX_ENC_LEN_NO_VALUE: usize = FieldHeader::MAX_ENCODED_LEN + VarIntSize::MAX_ENCODED_LEN;

/// A field of a message.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MessageField<'a, M: Message> {
    field_number: u32,
    value: Option<&'a M>,
}

impl<'a, M: Message> MessageField<'a, M> {
    //! Construction

    /// Creates a new message field.
    #[inline(always)]
    pub fn new(field_number: u32, value: Option<&'a M>) -> Self {
        debug_assert!(field_number != 0);

        Self {
            field_number,
            value,
        }
    }
}

impl<'a, M: Message> MessageField<'a, M> {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(WireType::LengthPrefixed, self.field_number)
    }
}

impl<'a, M: Message> EncodedLen for MessageField<'a, M> {
    fn encoded_len(&self) -> usize {
        if let Some(value) = self.value {
            let encoded_len: usize = value.encoded_len();
            self.field_header().encoded_len()
                + VarIntSize::from(encoded_len).encoded_len()
                + encoded_len
        } else {
            0
        }
    }
}

impl<'a, M: Message> EncodeToSlice for MessageField<'a, M> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> usize {
        if let Some(value) = self.value {
            let encoded_len: usize = value.encoded_len();

            let header_len: usize = self.field_header().encode_to_slice_unchecked(target);
            let length_prefix_len: usize =
                VarIntSize::from(encoded_len).encode_to_slice_unchecked(&mut target[header_len..]);

            let start: usize = header_len + length_prefix_len;
            let also_encoded_len: usize = value.encode_to_slice_unchecked(&mut target[start..]);
            debug_assert_eq!(encoded_len, also_encoded_len);

            start + encoded_len
        } else {
            0
        }
    }
}

impl<'a, M: Message> EncodeToWrite for MessageField<'a, M> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        if let Some(value) = self.value {
            let mut buffer: [u8; MAX_ENC_LEN_NO_VALUE] = [0u8; MAX_ENC_LEN_NO_VALUE];
            unsafe {
                let encoded_len: usize = value.encoded_len();
                let header_len: usize = self.field_header().encode_to_slice_unchecked(&mut buffer);
                let length_prefix_len: usize = VarIntSize::from(encoded_len)
                    .encode_to_slice_unchecked(&mut buffer[header_len..]);
                w.write_all(&buffer[..header_len + length_prefix_len])?;

                let also_encoded_len: usize = value.encode_to_write(w)?;
                debug_assert_eq!(encoded_len, also_encoded_len);

                Ok(header_len + length_prefix_len + encoded_len)
            }
        } else {
            Ok(0)
        }
    }
}
