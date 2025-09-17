use enc::Error;
use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Message, Packet, PacketType};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A message with named type fields.
/// message NamedTypes {
///    
///    // A `message` field.
///    one: fields.messages.PrimitiveTypes = 1;
///    
///    // A `struct` field.
///    two: fields.structs.PrimitiveTypes = 2;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct NamedTypes {
    packet_unrecognized_fields: Vec<u8>,
    one: Option<crate::fields::messages::PrimitiveTypes>,
    two: Option<crate::fields::structs::PrimitiveTypes>,
}

impl NamedTypes {
    //! Field: `one`
    //!
    //! // A `message` field.
    //! one: fields.messages.PrimitiveTypes = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<&crate::fields::messages::PrimitiveTypes> {
        self.one.as_ref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Option<crate::fields::messages::PrimitiveTypes>
    where
        T: Into<Option<crate::fields::messages::PrimitiveTypes>>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Option<crate::fields::messages::PrimitiveTypes>>,
    {
        self.set_one(one);
        self
    }
}

impl NamedTypes {
    //! Field: `two`
    //!
    //! // A `struct` field.
    //! two: fields.structs.PrimitiveTypes = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<&crate::fields::structs::PrimitiveTypes> {
        self.two.as_ref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> Option<crate::fields::structs::PrimitiveTypes>
    where
        T: Into<Option<crate::fields::structs::PrimitiveTypes>>,
    {
        std::mem::replace(&mut self.two, two.into())
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<Option<crate::fields::structs::PrimitiveTypes>>,
    {
        self.set_two(two);
        self
    }
}

impl Packet for NamedTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }

    fn packet_type() -> PacketType {
        PacketType::Message
    }
}

impl Message for NamedTypes {}

impl EncodedLen for NamedTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encoded_len!(
            &self.one,
            false,
            1,
            crate::fields::messages::PrimitiveTypes::wire_type(),
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.two,
            false,
            2,
            crate::fields::structs::PrimitiveTypes::wire_type(),
            encoded_len
        );

        Ok(encoded_len)
    }
}

impl EncodeToSlice for NamedTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.one,
            false,
            1,
            crate::fields::messages::PrimitiveTypes::wire_type(),
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.two,
            false,
            2,
            crate::fields::structs::PrimitiveTypes::wire_type(),
            encoded_len,
            &mut target[encoded_len..]
        );

        Ok(encoded_len)
    }
}

impl EncodeToWrite for NamedTypes {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encode_to_write!(
            &self.one,
            false,
            1,
            crate::fields::messages::PrimitiveTypes::wire_type(),
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.two,
            false,
            2,
            crate::fields::structs::PrimitiveTypes::wire_type(),
            encoded_len,
            w
        );

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
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::decode_from_read_prefix_with_first_byte(r, first)?;
            match header.tag_number().value() {
                1 => {
                    let value: crate::fields::messages::PrimitiveTypes = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_packet(
                            crate::fields::messages::PrimitiveTypes::wire_type(),
                            r,
                            first,
                        )?
                    };
                    result.set_one(value);
                }
                2 => {
                    let value: crate::fields::structs::PrimitiveTypes = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_packet(
                            crate::fields::structs::PrimitiveTypes::wire_type(),
                            r,
                            first,
                        )?
                    };
                    result.set_two(value);
                }
                _ => {
                    let mut w: std::io::Cursor<&mut Vec<u8>> =
                        std::io::Cursor::new(&mut result.packet_unrecognized_fields);
                    header.encode_to_write(&mut w)?;
                    header.wire_type().transfer(r, &mut w)?;
                }
            }
        }
        Ok(result)
    }
}

impl DecodeFromReadPrefix for NamedTypes {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
