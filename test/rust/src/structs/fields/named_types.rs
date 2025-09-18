use enc::Error;
use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Packet, PacketType, Struct};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A struct with named type fields.
/// struct NamedTypes {
///    
///    // A `struct` field.
///    one: structs.fields.PrimitiveTypes;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct NamedTypes {
    one: crate::structs::fields::PrimitiveTypes,
}

impl NamedTypes {
    //! Construction

    /// Creates a new `NamedTypes`.
    pub fn new(one: crate::structs::fields::PrimitiveTypes) -> Self {
        Self { one }
    }

    /// Creates a new `NamedTypes`.
    pub fn from<F0>(one: F0) -> Self
    where
        F0: Into<crate::structs::fields::PrimitiveTypes>,
    {
        Self { one: one.into() }
    }
}

impl NamedTypes {
    //! Field: `one`
    //!
    //! // A `struct` field.
    //! one: structs.fields.PrimitiveTypes;

    /// Gets the field: `one`.
    pub fn one(&self) -> &crate::structs::fields::PrimitiveTypes {
        &self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> crate::structs::fields::PrimitiveTypes
    where
        T: Into<crate::structs::fields::PrimitiveTypes>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<crate::structs::fields::PrimitiveTypes>,
    {
        self.set_one(one);
        self
    }
}

impl Packet for NamedTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }

    fn packet_type() -> PacketType {
        PacketType::Struct
    }
}

impl Struct for NamedTypes {}

impl EncodedLen for NamedTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        use proto_packet::io::Encoder;

        let mut encoded_len: usize = 0;

        proto_packet::impl_struct_field_encoded_len!(&self.one, false, encoded_len);

        Ok(encoded_len)
    }
}

impl EncodeToSlice for NamedTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        use proto_packet::io::Encoder;

        let mut encoded_len: usize = 0;

        proto_packet::impl_struct_field_encode_to_slice_unchecked!(
            &self.one,
            false,
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
        use proto_packet::io::Encoder;

        let mut encoded_len: usize = 0;

        proto_packet::impl_struct_field_encode_to_write!(&self.one, false, encoded_len, w);

        Ok(encoded_len)
    }
}

impl DecodeFromRead for NamedTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let decoded_one: crate::structs::fields::PrimitiveTypes = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_packet(
                crate::structs::fields::PrimitiveTypes::wire_type(),
                r,
                first,
            )?
        };

        debug_assert!(enc::read_optional_byte(r)?.is_none());

        Ok(Self { one: decoded_one })
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
