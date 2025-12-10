/// // A message with special types.
/// struct Specials {
///    
///    // A 'uuid' field.
///    one: uuid = 1;
///    
///    // A 'string' field.
///    two: string = 2;
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
pub struct Specials {
    one: Option<uuid::Uuid>,
    two: Option<String>,
}

impl proto_packet::Packet for Specials {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Message for Specials {}

impl Specials {
    //! Field: `one`
    //!
    //! // A 'uuid' field.
    //! one: uuid = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<uuid::Uuid> {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Option<uuid::Uuid>
    where
        T: Into<Option<uuid::Uuid>>,
    {
        let old_one: Option<uuid::Uuid> = self.one;
        self.one = one.into();
        old_one
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Option<uuid::Uuid>>,
    {
        self.set_one(one);
        self
    }
}

impl Specials {
    //! Field: `two`
    //!
    //! // A 'string' field.
    //! two: string = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<&str> {
        self.two.as_deref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> Option<String>
    where
        T: Into<Option<String>>,
    {
        std::mem::replace(&mut self.two, two.into())
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<Option<String>>,
    {
        self.set_two(two);
        self
    }
}

impl enc::EncodedLen for Specials {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::Fixed16Byte, tag);
            encoded_len += header.encoded_len()?;
            let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encoded_len()?;
        }

        if let Some(value) = &self.two {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::LengthPrefixed, tag);
            encoded_len += header.encoded_len()?;
            let encoder: proto_packet::io::Encoder<String> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encoded_len()?;
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Specials {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::Fixed16Byte, tag);
            encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }

        if let Some(value) = &self.two {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::LengthPrefixed, tag);
            encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            let encoder: proto_packet::io::Encoder<String> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToWrite for Specials {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, enc::Error>
    where
        W: std::io::Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::Fixed16Byte, tag);
            encoded_len += header.encode_to_write(w)?;
            let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_write(w)?;
        }

        if let Some(value) = &self.two {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::LengthPrefixed, tag);
            encoded_len += header.encode_to_write(w)?;
            let encoder: proto_packet::io::Encoder<String> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_write(w)?;
        }

        Ok(encoded_len)
    }
}

impl enc::DecodeFromRead for Specials {
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
                    let value: uuid::Uuid = {
                        let decoder: Decoder = Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_uuid(proto_packet::io::WireType::Fixed16Byte, r, first)?
                    };
                    result.set_one(value);
                }
                2 => {
                    let value: String = {
                        let decoder: Decoder = Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_string(
                            proto_packet::io::WireType::LengthPrefixed,
                            r,
                            first,
                        )?
                    };
                    result.set_two(value);
                }
                _ => {
                    todo!();
                }
            }
        }
        Ok(result)
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Specials);
