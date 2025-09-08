use enc::Error;
use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Packet, Struct};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A struct with special type fields.
/// struct SpecialTypes {
///    
///    // A `uuid` field.
///    one: uuid;
///    
///    // A `string` field.
///    two: string;
///    
///    // A `date` field.
///    three: date;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SpecialTypes {
    one: uuid::Uuid,
    two: String,
    three: chrono::NaiveDate,
}

impl SpecialTypes {
    //! Construction

    /// Creates a new `SpecialTypes`.
    pub fn new<F0, F1, F2>(one: F0, two: F1, three: F2) -> Self
    where
        F0: Into<uuid::Uuid>,
        F1: Into<String>,
        F2: Into<chrono::NaiveDate>,
    {
        Self {
            one: one.into(),
            two: two.into(),
            three: three.into(),
        }
    }
}

impl SpecialTypes {
    //! Field: `one`
    //!
    //! // A `uuid` field.
    //! one: uuid;

    /// Gets the field: `one`.
    pub fn one(&self) -> uuid::Uuid {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> uuid::Uuid
    where
        T: Into<uuid::Uuid>,
    {
        let old_one: uuid::Uuid = self.one;
        self.one = one.into();
        old_one
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<uuid::Uuid>,
    {
        self.set_one(one);
        self
    }
}

impl SpecialTypes {
    //! Field: `two`
    //!
    //! // A `string` field.
    //! two: string;

    /// Gets the field: `two`.
    pub fn two(&self) -> &str {
        self.two.as_ref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> String
    where
        T: Into<String>,
    {
        std::mem::replace(&mut self.two, two.into())
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<String>,
    {
        self.set_two(two);
        self
    }
}

impl SpecialTypes {
    //! Field: `three`
    //!
    //! // A `date` field.
    //! three: date;

    /// Gets the field: `three`.
    pub fn three(&self) -> chrono::NaiveDate {
        self.three
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<T>(&mut self, three: T) -> chrono::NaiveDate
    where
        T: Into<chrono::NaiveDate>,
    {
        let old_three: chrono::NaiveDate = self.three;
        self.three = three.into();
        old_three
    }

    /// Sets the field: `three`. Returns the struct itself.
    pub fn with_three<T>(mut self, three: T) -> Self
    where
        T: Into<chrono::NaiveDate>,
    {
        self.set_three(three);
        self
    }
}

impl Packet for SpecialTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Struct for SpecialTypes {}

impl EncodedLen for SpecialTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<String> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<chrono::NaiveDate> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encoded_len()?
        };

        Ok(encoded_len)
    }
}

impl EncodeToSlice for SpecialTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<String> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<chrono::NaiveDate> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        Ok(encoded_len)
    }
}

impl EncodeToWrite for SpecialTypes {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<String> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<chrono::NaiveDate> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encode_to_write(w)?
        };

        Ok(encoded_len)
    }
}

impl DecodeFromRead for SpecialTypes {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let decoded_one: uuid::Uuid = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_uuid(WireType::Fixed16Byte, r, first)?
        };

        let decoded_two: String = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_string(WireType::LengthPrefixed, r, first)?
        };

        let decoded_three: chrono::NaiveDate = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_date(WireType::VarInt, r, first)?
        };

        debug_assert!(enc::read_optional_byte(r)?.is_none());

        Ok(Self {
            one: decoded_one,
            two: decoded_two,
            three: decoded_three,
        })
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
