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
}

impl Primitives {
    //! Construction

    /// Creates a new `Primitives`.
    pub const fn new(one: u8, two: u16, three: u32, four: u64, five: u128) -> Self {
        Self {
            one,
            two,
            three,
            four,
            five,
        }
    }

    /// Creates a new `Primitives`.
    pub fn from<F0, F1, F2, F3, F4>(one: F0, two: F1, three: F2, four: F3, five: F4) -> Self
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

enc::impl_decode_from_read_prefix_length_prefixed!(Primitives);
