/// // A message with primitive type slices.
/// struct Primitives {
///    
///    // A 'u8' field.
///    one: []u8 = 1;
///    
///    // A 'u16' field.
///    two: []u16 = 2;
///    
///    // A 'u32' field.
///    three: []u32 = 3;
///    
///    // A 'u64' field.
///    four: []u64 = 4;
///    
///    // A 'u128' field.
///    five: []u128 = 5;
/// }
#[derive(
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Primitives {
    one: Option<Vec<u8>>,
    two: Option<Vec<u16>>,
    three: Option<Vec<u32>>,
    four: Option<Vec<u64>>,
    five: Option<Vec<u128>>,
}

impl proto_packet::Packet for Primitives {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Message for Primitives {}

impl Primitives {
    //! Field: `one`
    //!
    //! // A 'u8' field.
    //! one: []u8 = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<&[u8]> {
        self.one.as_deref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Option<Vec<u8>>
    where
        T: Into<Option<Vec<u8>>>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Option<Vec<u8>>>,
    {
        self.set_one(one);
        self
    }
}

impl Primitives {
    //! Field: `two`
    //!
    //! // A 'u16' field.
    //! two: []u16 = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<&[u16]> {
        self.two.as_deref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> Option<Vec<u16>>
    where
        T: Into<Option<Vec<u16>>>,
    {
        std::mem::replace(&mut self.two, two.into())
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<Option<Vec<u16>>>,
    {
        self.set_two(two);
        self
    }
}

impl Primitives {
    //! Field: `three`
    //!
    //! // A 'u32' field.
    //! three: []u32 = 3;

    /// Gets the field: `three`.
    pub fn three(&self) -> Option<&[u32]> {
        self.three.as_deref()
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<T>(&mut self, three: T) -> Option<Vec<u32>>
    where
        T: Into<Option<Vec<u32>>>,
    {
        std::mem::replace(&mut self.three, three.into())
    }

    /// Sets the field: `three`. Returns the struct itself.
    pub fn with_three<T>(mut self, three: T) -> Self
    where
        T: Into<Option<Vec<u32>>>,
    {
        self.set_three(three);
        self
    }
}

impl Primitives {
    //! Field: `four`
    //!
    //! // A 'u64' field.
    //! four: []u64 = 4;

    /// Gets the field: `four`.
    pub fn four(&self) -> Option<&[u64]> {
        self.four.as_deref()
    }

    /// Sets the field: `four`. Returns the previous value.
    pub fn set_four<T>(&mut self, four: T) -> Option<Vec<u64>>
    where
        T: Into<Option<Vec<u64>>>,
    {
        std::mem::replace(&mut self.four, four.into())
    }

    /// Sets the field: `four`. Returns the struct itself.
    pub fn with_four<T>(mut self, four: T) -> Self
    where
        T: Into<Option<Vec<u64>>>,
    {
        self.set_four(four);
        self
    }
}

impl Primitives {
    //! Field: `five`
    //!
    //! // A 'u128' field.
    //! five: []u128 = 5;

    /// Gets the field: `five`.
    pub fn five(&self) -> Option<&[u128]> {
        self.five.as_deref()
    }

    /// Sets the field: `five`. Returns the previous value.
    pub fn set_five<T>(&mut self, five: T) -> Option<Vec<u128>>
    where
        T: Into<Option<Vec<u128>>>,
    {
        std::mem::replace(&mut self.five, five.into())
    }

    /// Sets the field: `five`. Returns the struct itself.
    pub fn with_five<T>(mut self, five: T) -> Self
    where
        T: Into<Option<Vec<u128>>>,
    {
        self.set_five(five);
        self
    }
}

impl enc::EncodedLen for Primitives {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::LengthPrefixed, tag);
            encoded_len += header.encoded_len()?;
            let encoder: proto_packet::io::Encoder<Vec<u8>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encoded_len()?;
        }

        if let Some(value) = &self.two {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encoded_len()?;
            let encoder: proto_packet::io::Encoder<Vec<u16>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encoded_len()?;
        }

        if let Some(value) = &self.three {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encoded_len()?;
            let encoder: proto_packet::io::Encoder<Vec<u32>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encoded_len()?;
        }

        if let Some(value) = &self.four {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encoded_len()?;
            let encoder: proto_packet::io::Encoder<Vec<u64>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encoded_len()?;
        }

        if let Some(value) = &self.five {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encoded_len()?;
            let encoder: proto_packet::io::Encoder<Vec<u128>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encoded_len()?;
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Primitives {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::LengthPrefixed, tag);
            encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            let encoder: proto_packet::io::Encoder<Vec<u8>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }

        if let Some(value) = &self.two {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            let encoder: proto_packet::io::Encoder<Vec<u16>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }

        if let Some(value) = &self.three {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            let encoder: proto_packet::io::Encoder<Vec<u32>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }

        if let Some(value) = &self.four {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            let encoder: proto_packet::io::Encoder<Vec<u64>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }

        if let Some(value) = &self.five {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            let encoder: proto_packet::io::Encoder<Vec<u128>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToWrite for Primitives {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, enc::Error>
    where
        W: std::io::Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::LengthPrefixed, tag);
            encoded_len += header.encode_to_write(w)?;
            let encoder: proto_packet::io::Encoder<Vec<u8>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_write(w)?;
        }

        if let Some(value) = &self.two {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_write(w)?;
            let encoder: proto_packet::io::Encoder<Vec<u16>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_write(w)?;
        }

        if let Some(value) = &self.three {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(3) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_write(w)?;
            let encoder: proto_packet::io::Encoder<Vec<u32>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_write(w)?;
        }

        if let Some(value) = &self.four {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(4) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_write(w)?;
            let encoder: proto_packet::io::Encoder<Vec<u64>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_write(w)?;
        }

        if let Some(value) = &self.five {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(5) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_write(w)?;
            let encoder: proto_packet::io::Encoder<Vec<u128>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_write(w)?;
        }

        Ok(encoded_len)
    }
}

impl enc::DecodeFromRead for Primitives {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, enc::Error>
    where
        R: std::io::Read,
    {
        use enc::DecodeFromReadPrefix;
        use proto_packet::io::{Decoder, FieldHeader};

        let mut result: Self = Self::default();

        while let Some(first) = enc::read_optional_byte(r)? {
            let header: FieldHeader =
                FieldHeader::decode_from_read_prefix_with_first_byte(r, first)?;
            match header.tag().value() {
                1 => {
                    let value: Vec<u8> = {
                        let decoder: Decoder = Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u8_list(
                            proto_packet::io::WireType::LengthPrefixed,
                            r,
                            first,
                        )?
                    };
                    result.set_one(value);
                }
                2 => {
                    let value: Vec<u16> = {
                        let decoder: Decoder = Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u16_list(proto_packet::io::WireType::List, r, first)?
                    };
                    result.set_two(value);
                }
                3 => {
                    let value: Vec<u32> = {
                        let decoder: Decoder = Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u32_list(proto_packet::io::WireType::List, r, first)?
                    };
                    result.set_three(value);
                }
                4 => {
                    let value: Vec<u64> = {
                        let decoder: Decoder = Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u64_list(proto_packet::io::WireType::List, r, first)?
                    };
                    result.set_four(value);
                }
                5 => {
                    let value: Vec<u128> = {
                        let decoder: Decoder = Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_u128_list(proto_packet::io::WireType::List, r, first)?
                    };
                    result.set_five(value);
                }
                _ => {
                    todo!();
                }
            }
        }
        Ok(result)
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Primitives);
