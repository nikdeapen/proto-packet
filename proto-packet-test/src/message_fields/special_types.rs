use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::{FieldHeader, TagNumber, WireType};
use proto_packet::{Message, Packet};
use std::io::{Error, Read, Write};

/// // A message with special types.
/// message SpecialTypes {
///   
///   // The first field.
///   one: uuid = 1;
///   
///   // The second field.
///   two: string = 2;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct SpecialTypes {
    one: Option<uuid::Uuid>,
    two: Option<String>,
}

impl Packet for SpecialTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for SpecialTypes {}

impl SpecialTypes {
    //! Field `one`
    //!
    //! // The first field.
    //! one: uuid = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<uuid::Uuid> {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<O>(&mut self, one: O) -> Option<uuid::Uuid>
    where
        O: Into<Option<uuid::Uuid>>,
    {
        let old_one: Option<uuid::Uuid> = self.one;
        self.one = one.into();
        old_one
    }

    /// Builds the field: `one`. Returns the struct itself.
    pub fn with_one<O>(mut self, one: O) -> Self
    where
        O: Into<Option<uuid::Uuid>>,
    {
        self.one = one.into();
        self
    }
}

impl SpecialTypes {
    //! Field `two`
    //!
    //! // The second field.
    //! two: string = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<&str> {
        self.two.as_deref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<O>(&mut self, two: O) -> Option<String>
    where
        O: Into<Option<String>>,
    {
        let two: Option<String> = two.into();
        std::mem::replace(&mut self.two, two)
    }

    /// Builds the field: `two`. Returns the struct itself.
    pub fn with_two<O>(mut self, two: O) -> Self
    where
        O: Into<Option<String>>,
    {
        self.two = two.into();
        self
    }
}

impl EncodedLen for SpecialTypes {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize =
                proto_packet::io::Fixed16ByteField::from_uuid(tag_number, value).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.two {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(2) };
            let field_len: usize =
                proto_packet::io::BytesField::from_string(tag_number, value).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl EncodeToSlice for SpecialTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize = proto_packet::io::Fixed16ByteField::from_uuid(tag_number, value)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.two {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(2) };
            let field_len: usize = proto_packet::io::BytesField::from_string(tag_number, value)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl EncodeToWrite for SpecialTypes {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize = proto_packet::io::Fixed16ByteField::from_uuid(tag_number, value)
                .encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.two {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(2) };
            let field_len: usize =
                proto_packet::io::BytesField::from_string(tag_number, value).encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl DecodeFromRead for SpecialTypes {
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
                    let value: uuid::Uuid =
                        proto_packet::io::decode_uuid(field_header.wire_type(), r)?;
                    result.set_one(value);
                }
                2 => {
                    let value: String =
                        proto_packet::io::decode_string(field_header.wire_type(), r)?;
                    result.set_two(value);
                }
                _ => {}
            }
        }

        Ok(result)
    }
}

impl DecodeFromReadPrefix for SpecialTypes {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use DecodeFromRead;
        Self::decode_from_read_length_prefixed_with_first_byte(first, r)
    }
}
