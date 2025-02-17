use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::{FieldHeader, TagNumber, WireType};
use proto_packet::{Message, Packet};
use std::io::{Error, Read, Write};

/// // A message with named types.
/// message NamedTypes {
///   
///   // A local message field.
///   local_message: message_fields.UnsignedInts = 1;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct NamedTypes {
    local_message: Option<crate::message_fields::UnsignedInts>,
}

impl Packet for NamedTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for NamedTypes {}

impl NamedTypes {
    //! Field `local_message`
    //!
    //! // A local message field.
    //! local_message: message_fields.UnsignedInts = 1;

    /// Gets the field: `local_message`.
    pub fn local_message(&self) -> Option<&crate::message_fields::UnsignedInts> {
        self.local_message.as_ref()
    }

    /// Sets the field: `local_message`. Returns the previous value.
    pub fn set_local_message<O>(
        &mut self,
        local_message: O,
    ) -> Option<crate::message_fields::UnsignedInts>
    where
        O: Into<Option<crate::message_fields::UnsignedInts>>,
    {
        let local_message: Option<crate::message_fields::UnsignedInts> = local_message.into();
        std::mem::replace(&mut self.local_message, local_message)
    }

    /// Builds the field: `local_message`. Returns the struct itself.
    pub fn with_local_message<O>(mut self, local_message: O) -> Self
    where
        O: Into<Option<crate::message_fields::UnsignedInts>>,
    {
        self.local_message = local_message.into();
        self
    }
}

impl EncodedLen for NamedTypes {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.local_message {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize =
                proto_packet::io::PacketField::from_packet(tag_number, value).encoded_len()?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl EncodeToSlice for NamedTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.local_message {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize = proto_packet::io::PacketField::from_packet(tag_number, value)
                .encode_to_slice(&mut target[encoded_len..])?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl EncodeToWrite for NamedTypes {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.local_message {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked(1) };
            let field_len: usize =
                proto_packet::io::PacketField::from_packet(tag_number, value).encode_to_write(w)?;
            encoded_len = encoded_len
                .checked_add(field_len)
                .ok_or(enc::Error::IntegerOverflow)?;
        }

        Ok(encoded_len)
    }
}

impl DecodeFromRead for NamedTypes {
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
                    let value: crate::message_fields::UnsignedInts =
                        proto_packet::io::decode_packet(field_header.wire_type(), r)?;
                    result.set_local_message(value);
                }
                _ => {}
            }
        }

        Ok(result)
    }
}

impl DecodeFromReadPrefix for NamedTypes {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        use DecodeFromRead;
        Self::decode_from_read_length_prefixed_with_first_byte(first, r)
    }
}
