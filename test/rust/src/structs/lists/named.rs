/// // A struct with named type slices.
/// struct Named {
///    
///    // A 'Primitives' field.
///    one: []structs.lists.Primitives;
/// }
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Named {
    one: Vec<crate::structs::lists::Primitives>,
}

impl Named {
    //! Construction

    /// Creates a new `Named`.
    pub const fn new(one: Vec<crate::structs::lists::Primitives>) -> Self {
        Self { one }
    }

    /// Creates a new `Named`.
    pub fn from<F0>(one: F0) -> Self
    where
        F0: Into<Vec<crate::structs::lists::Primitives>>,
    {
        Self { one: one.into() }
    }
}

impl proto_packet::Packet for Named {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Struct for Named {}

impl Named {
    //! Field: `one`
    //!
    //! // A 'Primitives' field.
    //! one: []structs.lists.Primitives;

    /// Gets the field: `one`.
    pub fn one(&self) -> &[crate::structs::lists::Primitives] {
        self.one.as_ref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Vec<crate::structs::lists::Primitives>
    where
        T: Into<Vec<crate::structs::lists::Primitives>>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Vec<crate::structs::lists::Primitives>>,
    {
        self.set_one(one);
        self
    }
}

impl enc::EncodedLen for Named {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<Vec<crate::structs::lists::Primitives>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encoded_len()?;

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Named {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<Vec<crate::structs::lists::Primitives>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        Ok(encoded_len)
    }
}

impl enc::EncodeToWrite for Named {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, enc::Error>
    where
        W: std::io::Write,
    {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<Vec<crate::structs::lists::Primitives>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encode_to_write(w)?;

        Ok(encoded_len)
    }
}

impl enc::DecodeFromRead for Named {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, enc::Error>
    where
        R: std::io::Read,
    {
        use proto_packet::io::Decoder;

        let decoded_one: Vec<crate::structs::lists::Primitives> = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_packet_list(proto_packet::io::WireType::List, r, first)?
        };

        debug_assert!(enc::read_optional_byte(r)?.is_none());

        Ok(Self { one: decoded_one })
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Named);
