use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::{FieldHeader, TagNumber, WireType};
use proto_packet::{Message, Packet};
use std::io::{Error, Read, Write};

/// // A message with unsigned integers.
/// message UnsignedInts {
///   
///   // The first field.
///   one: u8 = 1;
///   
///   // The second field.
///   two: u16 = 2;
///   
///   // The third field.
///   three: u32 = 3;
///   
///   // The fourth field.
///   four: u64 = 4;
///   
///   // The fifth field.
///   five: u128 = 5;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct UnsignedInts {
    one: Option<u8>,
    two: Option<u16>,
    three: Option<u32>,
    four: Option<u64>,
    five: Option<u128>,
}

impl Packet for UnsignedInts {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for UnsignedInts {}

impl UnsignedInts {
    //! Field `one`
    //!
    //! // The first field.
    //! one: u8 = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<u8> {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<O>(&mut self, one: O) -> Option<u8>
    where
        O: Into<Option<u8>>,
    {
        let old_one: Option<u8> = self.one;
        self.one = one.into();
        old_one
    }

    /// Builds the field: `one`. Returns the struct itself.
    pub fn with_one<O>(mut self, one: O) -> Self
    where
        O: Into<Option<u8>>,
    {
        self.one = one.into();
        self
    }
}

impl UnsignedInts {
    //! Field `two`
    //!
    //! // The second field.
    //! two: u16 = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<u16> {
        self.two
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<O>(&mut self, two: O) -> Option<u16>
    where
        O: Into<Option<u16>>,
    {
        let old_two: Option<u16> = self.two;
        self.two = two.into();
        old_two
    }

    /// Builds the field: `two`. Returns the struct itself.
    pub fn with_two<O>(mut self, two: O) -> Self
    where
        O: Into<Option<u16>>,
    {
        self.two = two.into();
        self
    }
}

impl UnsignedInts {
    //! Field `three`
    //!
    //! // The third field.
    //! three: u32 = 3;

    /// Gets the field: `three`.
    pub fn three(&self) -> Option<u32> {
        self.three
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<O>(&mut self, three: O) -> Option<u32>
    where
        O: Into<Option<u32>>,
    {
        let old_three: Option<u32> = self.three;
        self.three = three.into();
        old_three
    }

    /// Builds the field: `three`. Returns the struct itself.
    pub fn with_three<O>(mut self, three: O) -> Self
    where
        O: Into<Option<u32>>,
    {
        self.three = three.into();
        self
    }
}

impl UnsignedInts {
    //! Field `four`
    //!
    //! // The fourth field.
    //! four: u64 = 4;

    /// Gets the field: `four`.
    pub fn four(&self) -> Option<u64> {
        self.four
    }

    /// Sets the field: `four`. Returns the previous value.
    pub fn set_four<O>(&mut self, four: O) -> Option<u64>
    where
        O: Into<Option<u64>>,
    {
        let old_four: Option<u64> = self.four;
        self.four = four.into();
        old_four
    }

    /// Builds the field: `four`. Returns the struct itself.
    pub fn with_four<O>(mut self, four: O) -> Self
    where
        O: Into<Option<u64>>,
    {
        self.four = four.into();
        self
    }
}

impl UnsignedInts {
    //! Field `five`
    //!
    //! // The fifth field.
    //! five: u128 = 5;

    /// Gets the field: `five`.
    pub fn five(&self) -> Option<u128> {
        self.five
    }

    /// Sets the field: `five`. Returns the previous value.
    pub fn set_five<O>(&mut self, five: O) -> Option<u128>
    where
        O: Into<Option<u128>>,
    {
        let old_five: Option<u128> = self.five;
        self.five = five.into();
        old_five
    }

    /// Builds the field: `five`. Returns the struct itself.
    pub fn with_five<O>(mut self, five: O) -> Self
    where
        O: Into<Option<u128>>,
    {
        self.five = five.into();
        self
    }
}

impl EncodedLen for UnsignedInts {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize =
                proto_packet::io::Fixed1ByteField::from_u8(tag_number, value).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.two {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(2) };
            let field_len: usize =
                proto_packet::io::VarInt16Field::from_u16(tag_number, value).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.three {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(3) };
            let field_len: usize =
                proto_packet::io::VarInt32Field::from_u32(tag_number, value).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.four {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(4) };
            let field_len: usize =
                proto_packet::io::VarInt64Field::from_u64(tag_number, value).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.five {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(5) };
            let field_len: usize =
                proto_packet::io::VarInt128Field::from_u128(tag_number, value).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl EncodeToSlice for UnsignedInts {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize = proto_packet::io::Fixed1ByteField::from_u8(tag_number, value)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.two {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(2) };
            let field_len: usize = proto_packet::io::VarInt16Field::from_u16(tag_number, value)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.three {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(3) };
            let field_len: usize = proto_packet::io::VarInt32Field::from_u32(tag_number, value)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.four {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(4) };
            let field_len: usize = proto_packet::io::VarInt64Field::from_u64(tag_number, value)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.five {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(5) };
            let field_len: usize = proto_packet::io::VarInt128Field::from_u128(tag_number, value)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl EncodeToWrite for UnsignedInts {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize =
                proto_packet::io::Fixed1ByteField::from_u8(tag_number, value).encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.two {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(2) };
            let field_len: usize =
                proto_packet::io::VarInt16Field::from_u16(tag_number, value).encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.three {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(3) };
            let field_len: usize =
                proto_packet::io::VarInt32Field::from_u32(tag_number, value).encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.four {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(4) };
            let field_len: usize =
                proto_packet::io::VarInt64Field::from_u64(tag_number, value).encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        if let Some(value) = &self.five {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(5) };
            let field_len: usize = proto_packet::io::VarInt128Field::from_u128(tag_number, value)
                .encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl DecodeFromRead for UnsignedInts {
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
                    let value: u8 = proto_packet::io::decode_u8(field_header.wire_type(), r)?;
                    result.set_one(value);
                }
                2 => {
                    let value: u16 = proto_packet::io::decode_u16(field_header.wire_type(), r)?;
                    result.set_two(value);
                }
                3 => {
                    let value: u32 = proto_packet::io::decode_u32(field_header.wire_type(), r)?;
                    result.set_three(value);
                }
                4 => {
                    let value: u64 = proto_packet::io::decode_u64(field_header.wire_type(), r)?;
                    result.set_four(value);
                }
                5 => {
                    let value: u128 = proto_packet::io::decode_u128(field_header.wire_type(), r)?;
                    result.set_five(value);
                }
                _ => {}
            }
        }

        Ok(result)
    }
}

impl DecodeFromReadPrefix for UnsignedInts {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use DecodeFromRead;
        Self::decode_from_read_length_prefixed_with_first_byte(first, r)
    }
}
