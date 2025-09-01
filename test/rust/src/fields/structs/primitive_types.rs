use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::{Packet, Struct};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A struct with primitive types.
/// struct PrimitiveTypes {
///    
///    // A `bool` field.
///    one: bool;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct PrimitiveTypes {
    one: bool,
}

impl PrimitiveTypes {
    //! Construction

    /// Creates a new `PrimitiveTypes`.
    pub fn new<F0>(one: F0) -> Self
    where
        F0: Into<bool>,
    {
        Self { one: one.into() }
    }
}

impl PrimitiveTypes {
    //! Field: `one`
    //!
    //! // A `bool` field.
    //! one: bool;

    /// Gets the field: `one`.
    pub fn one(&self) -> bool {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> bool
    where
        T: Into<bool>,
    {
        let old_one: bool = self.one;
        self.one = one.into();
        old_one
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<bool>,
    {
        self.set_one(one);
        self
    }
}

impl Packet for PrimitiveTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Struct for PrimitiveTypes {}

impl EncodedLen for PrimitiveTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<bool> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encoded_len()?
        };

        Ok(encoded_len)
    }
}

impl EncodeToSlice for PrimitiveTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<bool> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        Ok(encoded_len)
    }
}

impl EncodeToWrite for PrimitiveTypes {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<bool> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encode_to_write(w)?
        };

        Ok(encoded_len)
    }
}

impl DecodeFromRead for PrimitiveTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let decoded_one: bool = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_bool(WireType::Fixed1Byte, r, first)?
        };

        debug_assert!(enc::read_optional_byte(r)?.is_none());

        Ok(Self { one: decoded_one })
    }
}

impl DecodeFromReadPrefix for PrimitiveTypes {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, StreamError>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
