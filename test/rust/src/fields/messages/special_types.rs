use enc::Error;
use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Message, Packet, PacketType};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A message with special type fields.
/// message SpecialTypes {
///    
///    // A `uuid` field.
///    one: uuid = 1;
///    
///    // A `string` field.
///    two: string = 2;
///    
///    // A `date` field.
///    three: date = 3;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct SpecialTypes {
    packet_unrecognized_fields: Vec<u8>,
    one: Option<uuid::Uuid>,
    two: Option<String>,
    three: Option<chrono::NaiveDate>,
}

impl SpecialTypes {
    //! Field: `one`
    //!
    //! // A `uuid` field.
    //! one: uuid = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<uuid::Uuid> {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Option<uuid::Uuid>
    where
        T: Into<Option<uuid::Uuid>>,
    {
        let old_one: Option<uuid::Uuid> = self.one;
        self.one = one.into();
        old_one
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Option<uuid::Uuid>>,
    {
        self.set_one(one);
        self
    }
}

impl SpecialTypes {
    //! Field: `two`
    //!
    //! // A `string` field.
    //! two: string = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<&str> {
        self.two.as_deref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> Option<String>
    where
        T: Into<Option<String>>,
    {
        std::mem::replace(&mut self.two, two.into())
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<Option<String>>,
    {
        self.set_two(two);
        self
    }
}

impl SpecialTypes {
    //! Field: `three`
    //!
    //! // A `date` field.
    //! three: date = 3;

    /// Gets the field: `three`.
    pub fn three(&self) -> Option<chrono::NaiveDate> {
        self.three
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<T>(&mut self, three: T) -> Option<chrono::NaiveDate>
    where
        T: Into<Option<chrono::NaiveDate>>,
    {
        let old_three: Option<chrono::NaiveDate> = self.three;
        self.three = three.into();
        old_three
    }

    /// Sets the field: `three`. Returns the struct itself.
    pub fn with_three<T>(mut self, three: T) -> Self
    where
        T: Into<Option<chrono::NaiveDate>>,
    {
        self.set_three(three);
        self
    }
}

impl Packet for SpecialTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }

    fn packet_type() -> PacketType {
        PacketType::Message
    }
}

impl Message for SpecialTypes {}

impl EncodedLen for SpecialTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encoded_len!(
            &self.one,
            false,
            1,
            WireType::Fixed16Byte,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.two,
            false,
            2,
            WireType::LengthPrefixed,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.three,
            false,
            3,
            WireType::VarInt,
            encoded_len
        );

        Ok(encoded_len)
    }
}

impl EncodeToSlice for SpecialTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.one,
            false,
            1,
            WireType::Fixed16Byte,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.two,
            false,
            2,
            WireType::LengthPrefixed,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.three,
            false,
            3,
            WireType::VarInt,
            encoded_len,
            &mut target[encoded_len..]
        );

        Ok(encoded_len)
    }
}

impl EncodeToWrite for SpecialTypes {
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
            WireType::Fixed16Byte,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.two,
            false,
            2,
            WireType::LengthPrefixed,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.three,
            false,
            3,
            WireType::VarInt,
            encoded_len,
            w
        );

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
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::decode_from_read_prefix_with_first_byte(r, first)?;
            match header.tag_number().value() {
                1 => {
                    let value: uuid::Uuid = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_uuid(WireType::Fixed16Byte, r, first)?
                    };
                    result.set_one(value);
                }
                2 => {
                    let value: String = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_string(WireType::LengthPrefixed, r, first)?
                    };
                    result.set_two(value);
                }
                3 => {
                    let value: chrono::NaiveDate = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_date(WireType::VarInt, r, first)?
                    };
                    result.set_three(value);
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

impl DecodeFromReadPrefix for SpecialTypes {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
