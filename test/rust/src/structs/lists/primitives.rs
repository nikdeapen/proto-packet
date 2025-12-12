/// // A struct with primitive type slices.
/// struct Primitives {
///    
///    // A 'u8' field.
///    one: []u8;
///    
///    // A 'u16' field.
///    two: []u16;
///    
///    // A 'u32' field.
///    three: []u32;
///    
///    // A 'u64' field.
///    four: []u64;
///    
///    // A 'u128' field.
///    five: []u128;
/// }
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Primitives {
    one: Vec<u8>,
    two: Vec<u16>,
    three: Vec<u32>,
    four: Vec<u64>,
    five: Vec<u128>,
}

impl Primitives {
    //! Construction

    /// Creates a new `Primitives`.
    pub const fn new(
        one: Vec<u8>,
        two: Vec<u16>,
        three: Vec<u32>,
        four: Vec<u64>,
        five: Vec<u128>,
    ) -> Self {
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
        F0: Into<Vec<u8>>,
        F1: Into<Vec<u16>>,
        F2: Into<Vec<u32>>,
        F3: Into<Vec<u64>>,
        F4: Into<Vec<u128>>,
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
    //! one: []u8;

    /// Gets the field: `one`.
    pub fn one(&self) -> &[u8] {
        self.one.as_ref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Vec<u8>
    where
        T: Into<Vec<u8>>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        self.set_one(one);
        self
    }
}

impl Primitives {
    //! Field: `two`
    //!
    //! // A 'u16' field.
    //! two: []u16;

    /// Gets the field: `two`.
    pub fn two(&self) -> &[u16] {
        self.two.as_ref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> Vec<u16>
    where
        T: Into<Vec<u16>>,
    {
        std::mem::replace(&mut self.two, two.into())
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<Vec<u16>>,
    {
        self.set_two(two);
        self
    }
}

impl Primitives {
    //! Field: `three`
    //!
    //! // A 'u32' field.
    //! three: []u32;

    /// Gets the field: `three`.
    pub fn three(&self) -> &[u32] {
        self.three.as_ref()
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<T>(&mut self, three: T) -> Vec<u32>
    where
        T: Into<Vec<u32>>,
    {
        std::mem::replace(&mut self.three, three.into())
    }

    /// Sets the field: `three`. Returns the struct itself.
    pub fn with_three<T>(mut self, three: T) -> Self
    where
        T: Into<Vec<u32>>,
    {
        self.set_three(three);
        self
    }
}

impl Primitives {
    //! Field: `four`
    //!
    //! // A 'u64' field.
    //! four: []u64;

    /// Gets the field: `four`.
    pub fn four(&self) -> &[u64] {
        self.four.as_ref()
    }

    /// Sets the field: `four`. Returns the previous value.
    pub fn set_four<T>(&mut self, four: T) -> Vec<u64>
    where
        T: Into<Vec<u64>>,
    {
        std::mem::replace(&mut self.four, four.into())
    }

    /// Sets the field: `four`. Returns the struct itself.
    pub fn with_four<T>(mut self, four: T) -> Self
    where
        T: Into<Vec<u64>>,
    {
        self.set_four(four);
        self
    }
}

impl Primitives {
    //! Field: `five`
    //!
    //! // A 'u128' field.
    //! five: []u128;

    /// Gets the field: `five`.
    pub fn five(&self) -> &[u128] {
        self.five.as_ref()
    }

    /// Sets the field: `five`. Returns the previous value.
    pub fn set_five<T>(&mut self, five: T) -> Vec<u128>
    where
        T: Into<Vec<u128>>,
    {
        std::mem::replace(&mut self.five, five.into())
    }

    /// Sets the field: `five`. Returns the struct itself.
    pub fn with_five<T>(mut self, five: T) -> Self
    where
        T: Into<Vec<u128>>,
    {
        self.set_five(five);
        self
    }
}

impl enc::EncodedLen for Primitives {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<Vec<u8>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<Vec<u16>> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<Vec<u32>> =
            proto_packet::io::Encoder::new(&self.three, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<Vec<u64>> =
            proto_packet::io::Encoder::new(&self.four, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<Vec<u128>> =
            proto_packet::io::Encoder::new(&self.five, false);
        encoded_len += encoder.encoded_len()?;

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Primitives {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<Vec<u8>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<Vec<u16>> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<Vec<u32>> =
            proto_packet::io::Encoder::new(&self.three, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<Vec<u64>> =
            proto_packet::io::Encoder::new(&self.four, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<Vec<u128>> =
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

        let encoder: proto_packet::io::Encoder<Vec<u8>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<Vec<u16>> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<Vec<u32>> =
            proto_packet::io::Encoder::new(&self.three, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<Vec<u64>> =
            proto_packet::io::Encoder::new(&self.four, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<Vec<u128>> =
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

        let decoded_one: Vec<u8> = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u8_list(proto_packet::io::WireType::LengthPrefixed, r, first)?
        };

        let decoded_two: Vec<u16> = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u16_list(proto_packet::io::WireType::List, r, first)?
        };

        let decoded_three: Vec<u32> = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u32_list(proto_packet::io::WireType::List, r, first)?
        };

        let decoded_four: Vec<u64> = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u64_list(proto_packet::io::WireType::List, r, first)?
        };

        let decoded_five: Vec<u128> = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_u128_list(proto_packet::io::WireType::List, r, first)?
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
