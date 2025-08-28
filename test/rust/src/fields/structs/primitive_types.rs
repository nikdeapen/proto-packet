use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::{Packet, Struct};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A struct with primitive type fields.
/// struct PrimitiveTypes {
///    
///    // A `u8` field.
///    one: u8;
///    
///    // A `u16` field.
///    two: u16;
///    
///    // A `u32` field.
///    three: u32;
///    
///    // A `u64` field.
///    four: u64;
///    
///    // A `u128` field.
///    five: u128;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct PrimitiveTypes {
    one: u8,
    two: u16,
    three: u32,
    four: u64,
    five: u128,
}

impl PrimitiveTypes {
    //! Construction

    /// Creates a new `PrimitiveTypes`.
    pub fn new<F0, F1, F2, F3, F4>(one: F0, two: F1, three: F2, four: F3, five: F4) -> Self
    where
        F0: Into<u8>,
        F1: Into<u16>,
        F2: Into<u32>,
        F3: Into<u64>,
        F4: Into<u128>,
    {
        Self {
            one: one.into(),
            two: two.into(),
            three: three.into(),
            four: four.into(),
            five: five.into(),
        }
    }
}

impl PrimitiveTypes {
    //! Field: `one`
    //!
    //! // A `u8` field.
    //! one: u8;

    /// Gets the field: `one`.
    pub fn one(&self) -> u8 {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> u8
    where
        T: Into<u8>,
    {
        let old_one: u8 = self.one;
        self.one = one.into();
        old_one
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<u8>,
    {
        self.set_one(one);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `two`
    //!
    //! // A `u16` field.
    //! two: u16;

    /// Gets the field: `two`.
    pub fn two(&self) -> u16 {
        self.two
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> u16
    where
        T: Into<u16>,
    {
        let old_two: u16 = self.two;
        self.two = two.into();
        old_two
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<u16>,
    {
        self.set_two(two);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `three`
    //!
    //! // A `u32` field.
    //! three: u32;

    /// Gets the field: `three`.
    pub fn three(&self) -> u32 {
        self.three
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<T>(&mut self, three: T) -> u32
    where
        T: Into<u32>,
    {
        let old_three: u32 = self.three;
        self.three = three.into();
        old_three
    }

    /// Sets the field: `three`. Returns the struct itself.
    pub fn with_three<T>(mut self, three: T) -> Self
    where
        T: Into<u32>,
    {
        self.set_three(three);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `four`
    //!
    //! // A `u64` field.
    //! four: u64;

    /// Gets the field: `four`.
    pub fn four(&self) -> u64 {
        self.four
    }

    /// Sets the field: `four`. Returns the previous value.
    pub fn set_four<T>(&mut self, four: T) -> u64
    where
        T: Into<u64>,
    {
        let old_four: u64 = self.four;
        self.four = four.into();
        old_four
    }

    /// Sets the field: `four`. Returns the struct itself.
    pub fn with_four<T>(mut self, four: T) -> Self
    where
        T: Into<u64>,
    {
        self.set_four(four);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `five`
    //!
    //! // A `u128` field.
    //! five: u128;

    /// Gets the field: `five`.
    pub fn five(&self) -> u128 {
        self.five
    }

    /// Sets the field: `five`. Returns the previous value.
    pub fn set_five<T>(&mut self, five: T) -> u128
    where
        T: Into<u128>,
    {
        let old_five: u128 = self.five;
        self.five = five.into();
        old_five
    }

    /// Sets the field: `five`. Returns the struct itself.
    pub fn with_five<T>(mut self, five: T) -> Self
    where
        T: Into<u128>,
    {
        self.set_five(five);
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
            let encoder: proto_packet::io::Encoder<u8> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u16> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u32> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u64> =
                proto_packet::io::Encoder::new(&self.four, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u128> =
                proto_packet::io::Encoder::new(&self.five, false);
            encoder.encoded_len()?
        };

        Ok(encoded_len)
    }
}

impl EncodeToSlice for PrimitiveTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u8> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u16> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u32> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u64> =
                proto_packet::io::Encoder::new(&self.four, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u128> =
                proto_packet::io::Encoder::new(&self.five, false);
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
            let encoder: proto_packet::io::Encoder<u8> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u16> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u32> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u64> =
                proto_packet::io::Encoder::new(&self.four, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<u128> =
                proto_packet::io::Encoder::new(&self.five, false);
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
        let decoded_one: u8 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u8(WireType::Fixed1Byte, r, first)?
        };

        let decoded_two: u16 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u16(WireType::VarInt, r, first)?
        };

        let decoded_three: u32 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u32(WireType::VarInt, r, first)?
        };

        let decoded_four: u64 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u64(WireType::VarInt, r, first)?
        };

        let decoded_five: u128 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u128(WireType::VarInt, r, first)?
        };

        debug_assert!(enc::read_optional_byte(r)?.is_none());

        Ok(Self {
            one: decoded_one,
            two: decoded_two,
            three: decoded_three,
            four: decoded_four,
            five: decoded_five,
        })
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
