/// // A struct with named types.
/// struct Named {
///    
///    // A 'Primitives' field.
///    one: structs.fields.Primitives;
/// }
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Named {
    one: crate::structs::fields::Primitives,
}

impl Named {
    //! Construction

    /// Creates a new `Named`.
    pub const fn new(one: crate::structs::fields::Primitives) -> Self {
        Self { one }
    }

    /// Creates a new `Named`.
    pub fn from<F0>(one: F0) -> Self
    where
        F0: Into<crate::structs::fields::Primitives>,
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
    //! one: structs.fields.Primitives;

    /// Gets the field: `one`.
    pub fn one(&self) -> &crate::structs::fields::Primitives {
        &self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> crate::structs::fields::Primitives
    where
        T: Into<crate::structs::fields::Primitives>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<crate::structs::fields::Primitives>,
    {
        self.set_one(one);
        self
    }
}

impl enc::EncodedLen for Named {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<crate::structs::fields::Primitives> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encoded_len()?;

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Named {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<crate::structs::fields::Primitives> =
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

        let encoder: proto_packet::io::Encoder<crate::structs::fields::Primitives> =
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
        use proto_packet::Packet;

        let decoded_one: crate::structs::fields::Primitives = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_packet(crate::structs::fields::Primitives::wire_type(), r, first)?
        };

        debug_assert!(enc::read_optional_byte(r)?.is_none());

        Ok(Self { one: decoded_one })
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Named);
