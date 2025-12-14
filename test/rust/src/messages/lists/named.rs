/// // A message with named type slices.
/// struct Named {
///    
///    // A 'Primitives' field.
///    one: []messages.lists.Primitives = 1;
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
pub struct Named {
    one: Option<Vec<crate::messages::lists::Primitives>>,
}

impl proto_packet::Packet for Named {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Message for Named {}

impl Named {
    //! Field: `one`
    //!
    //! // A 'Primitives' field.
    //! one: []messages.lists.Primitives = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<&[crate::messages::lists::Primitives]> {
        self.one.as_deref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Option<Vec<crate::messages::lists::Primitives>>
    where
        T: Into<Option<Vec<crate::messages::lists::Primitives>>>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Option<Vec<crate::messages::lists::Primitives>>>,
    {
        self.set_one(one);
        self
    }
}

impl enc::EncodedLen for Named {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encoded_len()?;
            let encoder: proto_packet::io::Encoder<Vec<crate::messages::lists::Primitives>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encoded_len()?;
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Named {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            let encoder: proto_packet::io::Encoder<Vec<crate::messages::lists::Primitives>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToWrite for Named {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, enc::Error>
    where
        W: std::io::Write,
    {
        let mut encoded_len: usize = 0;

        if let Some(value) = &self.one {
            let tag: proto_packet::io::TagNumber =
                unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
            let header: proto_packet::io::FieldHeader =
                proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
            encoded_len += header.encode_to_write(w)?;
            let encoder: proto_packet::io::Encoder<Vec<crate::messages::lists::Primitives>> =
                proto_packet::io::Encoder::new(value, false);
            encoded_len += encoder.encode_to_write(w)?;
        }

        Ok(encoded_len)
    }
}

impl enc::DecodeFromRead for Named {
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
                    let value: Vec<crate::messages::lists::Primitives> = {
                        let decoder: Decoder = Decoder::default();
                        let first: u8 = enc::read_single_byte(r)?;
                        decoder.decode_packet_list(proto_packet::io::WireType::List, r, first)?
                    };
                    result.set_one(value);
                }
                _ => {
                    todo!();
                }
            }
        }
        Ok(result)
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Named);
