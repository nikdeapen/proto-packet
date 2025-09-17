use enc::Error;
use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use proto_packet::io::WireType;
use proto_packet::{Message, Packet, PacketType};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// // A message with primitive types.
/// message PrimitiveTypes {
///    
///    // A `u8` field.
///    one: u8 = 1;
///    
///    // A `u16` field.
///    two: u16 = 2;
///    
///    // A `u32` field.
///    three: u32 = 3;
///    
///    // A `u64` field.
///    four: u64 = 4;
///    
///    // A `u128` field.
///    five: u128 = 5;
///    
///    // An `i8` field.
///    six: i8 = 6;
///    
///    // An `i16` field.
///    seven: i16 = 7;
///    
///    // An `i32` field.
///    eight: i32 = 8;
///    
///    // An `i64` field.
///    nine: i64 = 9;
///    
///    // An `i128` field.
///    ten: i128 = 10;
///    
///    // A `bool` field.
///    eleven: bool = 11;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct PrimitiveTypes {
    packet_unrecognized_fields: Vec<u8>,
    one: Option<u8>,
    two: Option<u16>,
    three: Option<u32>,
    four: Option<u64>,
    five: Option<u128>,
    six: Option<i8>,
    seven: Option<i16>,
    eight: Option<i32>,
    nine: Option<i64>,
    ten: Option<i128>,
    eleven: Option<bool>,
}

impl PrimitiveTypes {
    //! Field: `one`
    //!
    //! // A `u8` field.
    //! one: u8 = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<u8> {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Option<u8>
    where
        T: Into<Option<u8>>,
    {
        let old_one: Option<u8> = self.one;
        self.one = one.into();
        old_one
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Option<u8>>,
    {
        self.set_one(one);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `two`
    //!
    //! // A `u16` field.
    //! two: u16 = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<u16> {
        self.two
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> Option<u16>
    where
        T: Into<Option<u16>>,
    {
        let old_two: Option<u16> = self.two;
        self.two = two.into();
        old_two
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<Option<u16>>,
    {
        self.set_two(two);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `three`
    //!
    //! // A `u32` field.
    //! three: u32 = 3;

    /// Gets the field: `three`.
    pub fn three(&self) -> Option<u32> {
        self.three
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<T>(&mut self, three: T) -> Option<u32>
    where
        T: Into<Option<u32>>,
    {
        let old_three: Option<u32> = self.three;
        self.three = three.into();
        old_three
    }

    /// Sets the field: `three`. Returns the struct itself.
    pub fn with_three<T>(mut self, three: T) -> Self
    where
        T: Into<Option<u32>>,
    {
        self.set_three(three);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `four`
    //!
    //! // A `u64` field.
    //! four: u64 = 4;

    /// Gets the field: `four`.
    pub fn four(&self) -> Option<u64> {
        self.four
    }

    /// Sets the field: `four`. Returns the previous value.
    pub fn set_four<T>(&mut self, four: T) -> Option<u64>
    where
        T: Into<Option<u64>>,
    {
        let old_four: Option<u64> = self.four;
        self.four = four.into();
        old_four
    }

    /// Sets the field: `four`. Returns the struct itself.
    pub fn with_four<T>(mut self, four: T) -> Self
    where
        T: Into<Option<u64>>,
    {
        self.set_four(four);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `five`
    //!
    //! // A `u128` field.
    //! five: u128 = 5;

    /// Gets the field: `five`.
    pub fn five(&self) -> Option<u128> {
        self.five
    }

    /// Sets the field: `five`. Returns the previous value.
    pub fn set_five<T>(&mut self, five: T) -> Option<u128>
    where
        T: Into<Option<u128>>,
    {
        let old_five: Option<u128> = self.five;
        self.five = five.into();
        old_five
    }

    /// Sets the field: `five`. Returns the struct itself.
    pub fn with_five<T>(mut self, five: T) -> Self
    where
        T: Into<Option<u128>>,
    {
        self.set_five(five);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `six`
    //!
    //! // An `i8` field.
    //! six: i8 = 6;

    /// Gets the field: `six`.
    pub fn six(&self) -> Option<i8> {
        self.six
    }

    /// Sets the field: `six`. Returns the previous value.
    pub fn set_six<T>(&mut self, six: T) -> Option<i8>
    where
        T: Into<Option<i8>>,
    {
        let old_six: Option<i8> = self.six;
        self.six = six.into();
        old_six
    }

    /// Sets the field: `six`. Returns the struct itself.
    pub fn with_six<T>(mut self, six: T) -> Self
    where
        T: Into<Option<i8>>,
    {
        self.set_six(six);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `seven`
    //!
    //! // An `i16` field.
    //! seven: i16 = 7;

    /// Gets the field: `seven`.
    pub fn seven(&self) -> Option<i16> {
        self.seven
    }

    /// Sets the field: `seven`. Returns the previous value.
    pub fn set_seven<T>(&mut self, seven: T) -> Option<i16>
    where
        T: Into<Option<i16>>,
    {
        let old_seven: Option<i16> = self.seven;
        self.seven = seven.into();
        old_seven
    }

    /// Sets the field: `seven`. Returns the struct itself.
    pub fn with_seven<T>(mut self, seven: T) -> Self
    where
        T: Into<Option<i16>>,
    {
        self.set_seven(seven);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `eight`
    //!
    //! // An `i32` field.
    //! eight: i32 = 8;

    /// Gets the field: `eight`.
    pub fn eight(&self) -> Option<i32> {
        self.eight
    }

    /// Sets the field: `eight`. Returns the previous value.
    pub fn set_eight<T>(&mut self, eight: T) -> Option<i32>
    where
        T: Into<Option<i32>>,
    {
        let old_eight: Option<i32> = self.eight;
        self.eight = eight.into();
        old_eight
    }

    /// Sets the field: `eight`. Returns the struct itself.
    pub fn with_eight<T>(mut self, eight: T) -> Self
    where
        T: Into<Option<i32>>,
    {
        self.set_eight(eight);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `nine`
    //!
    //! // An `i64` field.
    //! nine: i64 = 9;

    /// Gets the field: `nine`.
    pub fn nine(&self) -> Option<i64> {
        self.nine
    }

    /// Sets the field: `nine`. Returns the previous value.
    pub fn set_nine<T>(&mut self, nine: T) -> Option<i64>
    where
        T: Into<Option<i64>>,
    {
        let old_nine: Option<i64> = self.nine;
        self.nine = nine.into();
        old_nine
    }

    /// Sets the field: `nine`. Returns the struct itself.
    pub fn with_nine<T>(mut self, nine: T) -> Self
    where
        T: Into<Option<i64>>,
    {
        self.set_nine(nine);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `ten`
    //!
    //! // An `i128` field.
    //! ten: i128 = 10;

    /// Gets the field: `ten`.
    pub fn ten(&self) -> Option<i128> {
        self.ten
    }

    /// Sets the field: `ten`. Returns the previous value.
    pub fn set_ten<T>(&mut self, ten: T) -> Option<i128>
    where
        T: Into<Option<i128>>,
    {
        let old_ten: Option<i128> = self.ten;
        self.ten = ten.into();
        old_ten
    }

    /// Sets the field: `ten`. Returns the struct itself.
    pub fn with_ten<T>(mut self, ten: T) -> Self
    where
        T: Into<Option<i128>>,
    {
        self.set_ten(ten);
        self
    }
}

impl PrimitiveTypes {
    //! Field: `eleven`
    //!
    //! // A `bool` field.
    //! eleven: bool = 11;

    /// Gets the field: `eleven`.
    pub fn eleven(&self) -> Option<bool> {
        self.eleven
    }

    /// Sets the field: `eleven`. Returns the previous value.
    pub fn set_eleven<T>(&mut self, eleven: T) -> Option<bool>
    where
        T: Into<Option<bool>>,
    {
        let old_eleven: Option<bool> = self.eleven;
        self.eleven = eleven.into();
        old_eleven
    }

    /// Sets the field: `eleven`. Returns the struct itself.
    pub fn with_eleven<T>(mut self, eleven: T) -> Self
    where
        T: Into<Option<bool>>,
    {
        self.set_eleven(eleven);
        self
    }
}

impl Packet for PrimitiveTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }

    fn packet_type() -> PacketType {
        PacketType::Message
    }
}

impl Message for PrimitiveTypes {}

impl EncodedLen for PrimitiveTypes {
    fn encoded_len(&self) -> Result<usize, Error> {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encoded_len!(
            &self.one,
            false,
            1,
            WireType::Fixed1Byte,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.two,
            false,
            2,
            WireType::VarInt,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.three,
            false,
            3,
            WireType::VarInt,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.four,
            false,
            4,
            WireType::VarInt,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.five,
            false,
            5,
            WireType::VarInt,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.six,
            false,
            6,
            WireType::Fixed1Byte,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.seven,
            false,
            7,
            WireType::VarInt,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.eight,
            false,
            8,
            WireType::VarInt,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.nine,
            false,
            9,
            WireType::VarInt,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.ten,
            false,
            10,
            WireType::VarInt,
            encoded_len
        );
        proto_packet::impl_message_field_encoded_len!(
            &self.eleven,
            false,
            11,
            WireType::Fixed1Byte,
            encoded_len
        );

        Ok(encoded_len)
    }
}

impl EncodeToSlice for PrimitiveTypes {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        use proto_packet::io::{Encoder, FieldHeader, TagNumber};

        let mut encoded_len: usize = 0;

        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.one,
            false,
            1,
            WireType::Fixed1Byte,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.two,
            false,
            2,
            WireType::VarInt,
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
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.four,
            false,
            4,
            WireType::VarInt,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.five,
            false,
            5,
            WireType::VarInt,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.six,
            false,
            6,
            WireType::Fixed1Byte,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.seven,
            false,
            7,
            WireType::VarInt,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.eight,
            false,
            8,
            WireType::VarInt,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.nine,
            false,
            9,
            WireType::VarInt,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.ten,
            false,
            10,
            WireType::VarInt,
            encoded_len,
            &mut target[encoded_len..]
        );
        proto_packet::impl_message_field_encode_to_slice_unchecked!(
            &self.eleven,
            false,
            11,
            WireType::Fixed1Byte,
            encoded_len,
            &mut target[encoded_len..]
        );

        Ok(encoded_len)
    }
}

impl EncodeToWrite for PrimitiveTypes {
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
            WireType::Fixed1Byte,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.two,
            false,
            2,
            WireType::VarInt,
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
        proto_packet::impl_message_field_encode_to_write!(
            &self.four,
            false,
            4,
            WireType::VarInt,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.five,
            false,
            5,
            WireType::VarInt,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.six,
            false,
            6,
            WireType::Fixed1Byte,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.seven,
            false,
            7,
            WireType::VarInt,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.eight,
            false,
            8,
            WireType::VarInt,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.nine,
            false,
            9,
            WireType::VarInt,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.ten,
            false,
            10,
            WireType::VarInt,
            encoded_len,
            w
        );
        proto_packet::impl_message_field_encode_to_write!(
            &self.eleven,
            false,
            11,
            WireType::Fixed1Byte,
            encoded_len,
            w
        );

        Ok(encoded_len)
    }
}

impl DecodeFromRead for PrimitiveTypes {
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
                    let value: u8 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u8(WireType::Fixed1Byte, r, first)?
                    };
                    result.set_one(value);
                }
                2 => {
                    let value: u16 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u16(WireType::VarInt, r, first)?
                    };
                    result.set_two(value);
                }
                3 => {
                    let value: u32 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u32(WireType::VarInt, r, first)?
                    };
                    result.set_three(value);
                }
                4 => {
                    let value: u64 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u64(WireType::VarInt, r, first)?
                    };
                    result.set_four(value);
                }
                5 => {
                    let value: u128 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u128(WireType::VarInt, r, first)?
                    };
                    result.set_five(value);
                }
                6 => {
                    let value: i8 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_i8(WireType::Fixed1Byte, r, first)?
                    };
                    result.set_six(value);
                }
                7 => {
                    let value: i16 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_i16(WireType::VarInt, r, first)?
                    };
                    result.set_seven(value);
                }
                8 => {
                    let value: i32 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_i32(WireType::VarInt, r, first)?
                    };
                    result.set_eight(value);
                }
                9 => {
                    let value: i64 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_i64(WireType::VarInt, r, first)?
                    };
                    result.set_nine(value);
                }
                10 => {
                    let value: i128 = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_i128(WireType::VarInt, r, first)?
                    };
                    result.set_ten(value);
                }
                11 => {
                    let value: bool = {
                        let decoder: proto_packet::io::Decoder =
                            proto_packet::io::Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_bool(WireType::Fixed1Byte, r, first)?
                    };
                    result.set_eleven(value);
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

impl DecodeFromReadPrefix for PrimitiveTypes {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }
}
