use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::{Packet, Struct};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A struct with signed integers.
/// struct SignedInts {
///    
///    // An `i8` field.
///    one: i8;
///    
///    // An `i16` field.
///    two: i16;
///    
///    // An `i32` field.
///    three: i32;
///    
///    // An `i64` field.
///    four: i64;
///    
///    // An `i128` field.
///    five: i128;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SignedInts {
    one: i8,
    two: i16,
    three: i32,
    four: i64,
    five: i128,
}

impl SignedInts {
    //! Construction

    /// Creates a new `SignedInts`.
    pub fn new<F0, F1, F2, F3, F4>(one: F0, two: F1, three: F2, four: F3, five: F4) -> Self
    where
        F0: Into<i8>,
        F1: Into<i16>,
        F2: Into<i32>,
        F3: Into<i64>,
        F4: Into<i128>,
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

impl SignedInts {
    //! Field: `one`
    //!
    //! // An `i8` field.
    //! one: i8;

    /// Gets the field: `one`.
    pub fn one(&self) -> i8 {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> i8
    where
        T: Into<i8>,
    {
        let old_one: i8 = self.one;
        self.one = one.into();
        old_one
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<i8>,
    {
        self.set_one(one);
        self
    }
}

impl SignedInts {
    //! Field: `two`
    //!
    //! // An `i16` field.
    //! two: i16;

    /// Gets the field: `two`.
    pub fn two(&self) -> i16 {
        self.two
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> i16
    where
        T: Into<i16>,
    {
        let old_two: i16 = self.two;
        self.two = two.into();
        old_two
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<i16>,
    {
        self.set_two(two);
        self
    }
}

impl SignedInts {
    //! Field: `three`
    //!
    //! // An `i32` field.
    //! three: i32;

    /// Gets the field: `three`.
    pub fn three(&self) -> i32 {
        self.three
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<T>(&mut self, three: T) -> i32
    where
        T: Into<i32>,
    {
        let old_three: i32 = self.three;
        self.three = three.into();
        old_three
    }

    /// Sets the field: `three`. Returns the struct itself.
    pub fn with_three<T>(mut self, three: T) -> Self
    where
        T: Into<i32>,
    {
        self.set_three(three);
        self
    }
}

impl SignedInts {
    //! Field: `four`
    //!
    //! // An `i64` field.
    //! four: i64;

    /// Gets the field: `four`.
    pub fn four(&self) -> i64 {
        self.four
    }

    /// Sets the field: `four`. Returns the previous value.
    pub fn set_four<T>(&mut self, four: T) -> i64
    where
        T: Into<i64>,
    {
        let old_four: i64 = self.four;
        self.four = four.into();
        old_four
    }

    /// Sets the field: `four`. Returns the struct itself.
    pub fn with_four<T>(mut self, four: T) -> Self
    where
        T: Into<i64>,
    {
        self.set_four(four);
        self
    }
}

impl SignedInts {
    //! Field: `five`
    //!
    //! // An `i128` field.
    //! five: i128;

    /// Gets the field: `five`.
    pub fn five(&self) -> i128 {
        self.five
    }

    /// Sets the field: `five`. Returns the previous value.
    pub fn set_five<T>(&mut self, five: T) -> i128
    where
        T: Into<i128>,
    {
        let old_five: i128 = self.five;
        self.five = five.into();
        old_five
    }

    /// Sets the field: `five`. Returns the struct itself.
    pub fn with_five<T>(mut self, five: T) -> Self
    where
        T: Into<i128>,
    {
        self.set_five(five);
        self
    }
}

impl Packet for SignedInts {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Struct for SignedInts {}

impl EncodedLen for SignedInts {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i8> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i16> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i32> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i64> =
                proto_packet::io::Encoder::new(&self.four, false);
            encoder.encoded_len()?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i128> =
                proto_packet::io::Encoder::new(&self.five, false);
            encoder.encoded_len()?
        };

        Ok(encoded_len)
    }
}

impl EncodeToSlice for SignedInts {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i8> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i16> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i32> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i64> =
                proto_packet::io::Encoder::new(&self.four, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i128> =
                proto_packet::io::Encoder::new(&self.five, false);
            encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?
        };

        Ok(encoded_len)
    }
}

impl EncodeToWrite for SignedInts {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i8> =
                proto_packet::io::Encoder::new(&self.one, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i16> =
                proto_packet::io::Encoder::new(&self.two, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i32> =
                proto_packet::io::Encoder::new(&self.three, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i64> =
                proto_packet::io::Encoder::new(&self.four, false);
            encoder.encode_to_write(w)?
        };

        encoded_len += {
            let encoder: proto_packet::io::Encoder<i128> =
                proto_packet::io::Encoder::new(&self.five, false);
            encoder.encode_to_write(w)?
        };

        Ok(encoded_len)
    }
}

impl DecodeFromRead for SignedInts {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let decoded_one: i8 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i8(WireType::Fixed1Byte, r, first)?
        };

        let decoded_two: i16 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i16(WireType::VarInt, r, first)?
        };

        let decoded_three: i32 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i32(WireType::VarInt, r, first)?
        };

        let decoded_four: i64 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i64(WireType::VarInt, r, first)?
        };

        let decoded_five: i128 = {
            let decoder: proto_packet::io::Decoder = proto_packet::io::Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i128(WireType::VarInt, r, first)?
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

impl DecodeFromReadPrefix for SignedInts {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, StreamError>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
