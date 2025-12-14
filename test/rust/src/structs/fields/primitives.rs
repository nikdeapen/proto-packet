/// // A struct with primitive types.
/// struct Primitives {
///    
///    // A 'u8' field.
///    one: u8;
///    
///    // A 'u16' field.
///    two: u16;
///    
///    // A 'u32' field.
///    three: u32;
///    
///    // A 'u64' field.
///    four: u64;
///    
///    // A 'u128' field.
///    five: u128;
///    
///    // An 'i8' field.
///    six: i8;
///    
///    // An 'i16' field.
///    seven: i16;
///    
///    // An 'i32' field.
///    eight: i32;
///    
///    // An 'i64' field.
///    nine: i64;
///    
///    // An 'i128' field.
///    ten: i128;
/// }
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Primitives {
    one: u8,
    two: u16,
    three: u32,
    four: u64,
    five: u128,
    six: i8,
    seven: i16,
    eight: i32,
    nine: i64,
    ten: i128,
}

impl Primitives {
    //! Construction

    /// Creates a new `Primitives`.
    pub const fn new(
        one: u8,
        two: u16,
        three: u32,
        four: u64,
        five: u128,
        six: i8,
        seven: i16,
        eight: i32,
        nine: i64,
        ten: i128,
    ) -> Self {
        Self {
            one,
            two,
            three,
            four,
            five,
            six,
            seven,
            eight,
            nine,
            ten,
        }
    }

    /// Creates a new `Primitives`.
    pub fn from<F0, F1, F2, F3, F4, F5, F6, F7, F8, F9>(
        one: F0,
        two: F1,
        three: F2,
        four: F3,
        five: F4,
        six: F5,
        seven: F6,
        eight: F7,
        nine: F8,
        ten: F9,
    ) -> Self
    where
        F0: Into<u8>,
        F1: Into<u16>,
        F2: Into<u32>,
        F3: Into<u64>,
        F4: Into<u128>,
        F5: Into<i8>,
        F6: Into<i16>,
        F7: Into<i32>,
        F8: Into<i64>,
        F9: Into<i128>,
    {
        Self {
            one: one.into(),
            two: two.into(),
            three: three.into(),
            four: four.into(),
            five: five.into(),
            six: six.into(),
            seven: seven.into(),
            eight: eight.into(),
            nine: nine.into(),
            ten: ten.into(),
        }
    }
}

impl proto_packet::Packet for Primitives {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Struct for Primitives {}

impl Primitives {
    //! Field: `one`
    //!
    //! // A 'u8' field.
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

impl Primitives {
    //! Field: `two`
    //!
    //! // A 'u16' field.
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

impl Primitives {
    //! Field: `three`
    //!
    //! // A 'u32' field.
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

impl Primitives {
    //! Field: `four`
    //!
    //! // A 'u64' field.
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

impl Primitives {
    //! Field: `five`
    //!
    //! // A 'u128' field.
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

impl Primitives {
    //! Field: `six`
    //!
    //! // An 'i8' field.
    //! six: i8;

    /// Gets the field: `six`.
    pub fn six(&self) -> i8 {
        self.six
    }

    /// Sets the field: `six`. Returns the previous value.
    pub fn set_six<T>(&mut self, six: T) -> i8
    where
        T: Into<i8>,
    {
        let old_six: i8 = self.six;
        self.six = six.into();
        old_six
    }

    /// Sets the field: `six`. Returns the struct itself.
    pub fn with_six<T>(mut self, six: T) -> Self
    where
        T: Into<i8>,
    {
        self.set_six(six);
        self
    }
}

impl Primitives {
    //! Field: `seven`
    //!
    //! // An 'i16' field.
    //! seven: i16;

    /// Gets the field: `seven`.
    pub fn seven(&self) -> i16 {
        self.seven
    }

    /// Sets the field: `seven`. Returns the previous value.
    pub fn set_seven<T>(&mut self, seven: T) -> i16
    where
        T: Into<i16>,
    {
        let old_seven: i16 = self.seven;
        self.seven = seven.into();
        old_seven
    }

    /// Sets the field: `seven`. Returns the struct itself.
    pub fn with_seven<T>(mut self, seven: T) -> Self
    where
        T: Into<i16>,
    {
        self.set_seven(seven);
        self
    }
}

impl Primitives {
    //! Field: `eight`
    //!
    //! // An 'i32' field.
    //! eight: i32;

    /// Gets the field: `eight`.
    pub fn eight(&self) -> i32 {
        self.eight
    }

    /// Sets the field: `eight`. Returns the previous value.
    pub fn set_eight<T>(&mut self, eight: T) -> i32
    where
        T: Into<i32>,
    {
        let old_eight: i32 = self.eight;
        self.eight = eight.into();
        old_eight
    }

    /// Sets the field: `eight`. Returns the struct itself.
    pub fn with_eight<T>(mut self, eight: T) -> Self
    where
        T: Into<i32>,
    {
        self.set_eight(eight);
        self
    }
}

impl Primitives {
    //! Field: `nine`
    //!
    //! // An 'i64' field.
    //! nine: i64;

    /// Gets the field: `nine`.
    pub fn nine(&self) -> i64 {
        self.nine
    }

    /// Sets the field: `nine`. Returns the previous value.
    pub fn set_nine<T>(&mut self, nine: T) -> i64
    where
        T: Into<i64>,
    {
        let old_nine: i64 = self.nine;
        self.nine = nine.into();
        old_nine
    }

    /// Sets the field: `nine`. Returns the struct itself.
    pub fn with_nine<T>(mut self, nine: T) -> Self
    where
        T: Into<i64>,
    {
        self.set_nine(nine);
        self
    }
}

impl Primitives {
    //! Field: `ten`
    //!
    //! // An 'i128' field.
    //! ten: i128;

    /// Gets the field: `ten`.
    pub fn ten(&self) -> i128 {
        self.ten
    }

    /// Sets the field: `ten`. Returns the previous value.
    pub fn set_ten<T>(&mut self, ten: T) -> i128
    where
        T: Into<i128>,
    {
        let old_ten: i128 = self.ten;
        self.ten = ten.into();
        old_ten
    }

    /// Sets the field: `ten`. Returns the struct itself.
    pub fn with_ten<T>(mut self, ten: T) -> Self
    where
        T: Into<i128>,
    {
        self.set_ten(ten);
        self
    }
}

impl enc::EncodedLen for Primitives {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<u8> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<u16> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<u32> =
            proto_packet::io::Encoder::new(&self.three, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<u64> =
            proto_packet::io::Encoder::new(&self.four, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<u128> =
            proto_packet::io::Encoder::new(&self.five, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<i8> =
            proto_packet::io::Encoder::new(&self.six, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<i16> =
            proto_packet::io::Encoder::new(&self.seven, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<i32> =
            proto_packet::io::Encoder::new(&self.eight, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<i64> =
            proto_packet::io::Encoder::new(&self.nine, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<i128> =
            proto_packet::io::Encoder::new(&self.ten, false);
        encoded_len += encoder.encoded_len()?;

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Primitives {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<u8> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<u16> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<u32> =
            proto_packet::io::Encoder::new(&self.three, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<u64> =
            proto_packet::io::Encoder::new(&self.four, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<u128> =
            proto_packet::io::Encoder::new(&self.five, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<i8> =
            proto_packet::io::Encoder::new(&self.six, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<i16> =
            proto_packet::io::Encoder::new(&self.seven, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<i32> =
            proto_packet::io::Encoder::new(&self.eight, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<i64> =
            proto_packet::io::Encoder::new(&self.nine, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<i128> =
            proto_packet::io::Encoder::new(&self.ten, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        Ok(encoded_len)
    }
}

impl enc::EncodeToWrite for Primitives {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, enc::Error>
    where
        W: std::io::Write,
    {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<u8> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<u16> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<u32> =
            proto_packet::io::Encoder::new(&self.three, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<u64> =
            proto_packet::io::Encoder::new(&self.four, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<u128> =
            proto_packet::io::Encoder::new(&self.five, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<i8> =
            proto_packet::io::Encoder::new(&self.six, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<i16> =
            proto_packet::io::Encoder::new(&self.seven, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<i32> =
            proto_packet::io::Encoder::new(&self.eight, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<i64> =
            proto_packet::io::Encoder::new(&self.nine, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<i128> =
            proto_packet::io::Encoder::new(&self.ten, false);
        encoded_len += encoder.encode_to_write(w)?;

        Ok(encoded_len)
    }
}

impl enc::DecodeFromRead for Primitives {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, enc::Error>
    where
        R: std::io::Read,
    {
        use proto_packet::io::Decoder;

        let decoded_one: u8 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u8(proto_packet::io::WireType::Fixed1Byte, r, first)?
        };

        let decoded_two: u16 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u16(proto_packet::io::WireType::VarInt, r, first)?
        };

        let decoded_three: u32 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u32(proto_packet::io::WireType::VarInt, r, first)?
        };

        let decoded_four: u64 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u64(proto_packet::io::WireType::VarInt, r, first)?
        };

        let decoded_five: u128 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u128(proto_packet::io::WireType::VarInt, r, first)?
        };

        let decoded_six: i8 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i8(proto_packet::io::WireType::Fixed1Byte, r, first)?
        };

        let decoded_seven: i16 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i16(proto_packet::io::WireType::VarInt, r, first)?
        };

        let decoded_eight: i32 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i32(proto_packet::io::WireType::VarInt, r, first)?
        };

        let decoded_nine: i64 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i64(proto_packet::io::WireType::VarInt, r, first)?
        };

        let decoded_ten: i128 = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_i128(proto_packet::io::WireType::VarInt, r, first)?
        };

        debug_assert!(enc::read_optional_byte(r)?.is_none());

        Ok(Self {
            one: decoded_one,
            two: decoded_two,
            three: decoded_three,
            four: decoded_four,
            five: decoded_five,
            six: decoded_six,
            seven: decoded_seven,
            eight: decoded_eight,
            nine: decoded_nine,
            ten: decoded_ten,
        })
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Primitives);
