use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::{FieldHeader, TagNumber, WireType};
use proto_packet::{Message, Packet};
use std::io::{Error, Read, Write};

/// // A message with unsigned integer slices.
/// message MessageUnsignedIntSlices {
///   
///   // The first field.
///   one: []u8 = 1;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct MessageUnsignedIntSlices {
    one: Option<Vec<u8>>,
}

impl Packet for MessageUnsignedIntSlices {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for MessageUnsignedIntSlices {}

impl MessageUnsignedIntSlices {
    //! Field `one`
    //!
    //! // The first field.
    //! one: []u8 = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<&[u8]> {
        self.one.as_deref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<O>(&mut self, one: O) -> Option<Vec<u8>>
    where
        O: Into<Option<Vec<u8>>>,
    {
        let one: Option<Vec<u8>> = one.into();
        std::mem::replace(&mut self.one, one)
    }

    /// Builds the field: `one`. Returns the struct itself.
    pub fn with_one<O>(mut self, one: O) -> Self
    where
        O: Into<Option<Vec<u8>>>,
    {
        self.one = one.into();
        self
    }
}

impl EncodedLen for MessageUnsignedIntSlices {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_header_len: usize =
                FieldHeader::new(WireType::List, tag_number).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_header_len)
                .ok_or(enc::Error::IntegerOverflow)?;
            let list_size_bytes: usize = proto_packet::io::encoded_len_slice_u8(value)?;
            let list_header_len: usize =
                proto_packet::io::ListHeader::new(WireType::Fixed1Byte, list_size_bytes)
                    .encoded_len()?;
            encoded_len = encoded_len
                .checked_add(list_header_len)
                .ok_or(enc::Error::IntegerOverflow)?;
            let also_list_size_bytes: usize = proto_packet::io::encoded_len_slice_u8(value)?;
            debug_assert_eq!(list_size_bytes, also_list_size_bytes);
            let field_len: usize = list_size_bytes;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl EncodeToSlice for MessageUnsignedIntSlices {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_header_len: usize = FieldHeader::new(WireType::List, tag_number)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_header_len)
                .ok_or(enc::Error::IntegerOverflow)?;
            let list_size_bytes: usize = proto_packet::io::encoded_len_slice_u8(value)?;
            let list_header_len: usize =
                proto_packet::io::ListHeader::new(WireType::Fixed1Byte, list_size_bytes)
                    .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(list_header_len)
                .ok_or(enc::Error::IntegerOverflow)?;
            let also_list_size_bytes: usize =
                proto_packet::io::encode_to_slice_slice_u8(value, &mut target[encoded_len..])?;
            debug_assert_eq!(list_size_bytes, also_list_size_bytes);
            let field_len: usize = list_size_bytes;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl EncodeToWrite for MessageUnsignedIntSlices {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_header_len: usize =
                FieldHeader::new(WireType::List, tag_number).encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_header_len)
                .ok_or(enc::Error::IntegerOverflow)?;
            let list_size_bytes: usize = proto_packet::io::encoded_len_slice_u8(value)?;
            let list_header_len: usize =
                proto_packet::io::ListHeader::new(WireType::Fixed1Byte, list_size_bytes)
                    .encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(list_header_len)
                .ok_or(enc::Error::IntegerOverflow)?;
            let also_list_size_bytes: usize = proto_packet::io::encode_to_write_slice_u8(value, w)?;
            debug_assert_eq!(list_size_bytes, also_list_size_bytes);
            let field_len: usize = list_size_bytes;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl DecodeFromRead for MessageUnsignedIntSlices {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut result: Self = Self::default();

        while let Some(first) = enc::read_optional_byte(r)? {
            use enc::DecodeFromReadPrefix;
            let field_header: FieldHeader =
                FieldHeader::decode_from_read_prefix_with_first_byte(first, r)?;
            let tag_number: u32 = field_header.tag_number().tag_number();
            match tag_number {
                1 => {
                    let value: Vec<u8> =
                        proto_packet::io::decode_slice_u8(field_header.wire_type(), r)?;
                    result.set_one(value);
                }
                _ => {}
            }
        }

        Ok(result)
    }
}

impl DecodeFromReadPrefix for MessageUnsignedIntSlices {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use DecodeFromRead;
        Self::decode_from_read_length_prefixed_with_first_byte(first, r)
    }
}
