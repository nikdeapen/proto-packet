/// // A struct with special type slices.
/// struct Specials {
///    
///    // A 'uuid' field.
///    one: []uuid;
///    
///    // A 'string' field.
///    two: []string;
/// }
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Specials {
    one: Vec<uuid::Uuid>,
    two: Vec<String>,
}

impl Specials {
    //! Construction

    /// Creates a new `Specials`.
    pub const fn new(one: Vec<uuid::Uuid>, two: Vec<String>) -> Self {
        Self { one, two }
    }

    /// Creates a new `Specials`.
    pub fn from<F0, F1>(one: F0, two: F1) -> Self
    where
        F0: Into<Vec<uuid::Uuid>>,
        F1: Into<Vec<String>>,
    {
        Self {
            one: one.into(),
            two: two.into(),
        }
    }
}

impl proto_packet::Packet for Specials {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Struct for Specials {}

impl Specials {
    //! Field: `one`
    //!
    //! // A 'uuid' field.
    //! one: []uuid;

    /// Gets the field: `one`.
    pub fn one(&self) -> &[uuid::Uuid] {
        self.one.as_ref()
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<T>(&mut self, one: T) -> Vec<uuid::Uuid>
    where
        T: Into<Vec<uuid::Uuid>>,
    {
        std::mem::replace(&mut self.one, one.into())
    }

    /// Sets the field: `one`. Returns the struct itself.
    pub fn with_one<T>(mut self, one: T) -> Self
    where
        T: Into<Vec<uuid::Uuid>>,
    {
        self.set_one(one);
        self
    }
}

impl Specials {
    //! Field: `two`
    //!
    //! // A 'string' field.
    //! two: []string;

    /// Gets the field: `two`.
    pub fn two(&self) -> &[String] {
        self.two.as_ref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<T>(&mut self, two: T) -> Vec<String>
    where
        T: Into<Vec<String>>,
    {
        std::mem::replace(&mut self.two, two.into())
    }

    /// Sets the field: `two`. Returns the struct itself.
    pub fn with_two<T>(mut self, two: T) -> Self
    where
        T: Into<Vec<String>>,
    {
        self.set_two(two);
        self
    }
}

impl enc::EncodedLen for Specials {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encoded_len()?;

        let encoder: proto_packet::io::Encoder<Vec<String>> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encoded_len()?;

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Specials {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        let encoder: proto_packet::io::Encoder<Vec<String>> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;

        Ok(encoded_len)
    }
}

impl enc::EncodeToWrite for Specials {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, enc::Error>
    where
        W: std::io::Write,
    {
        let mut encoded_len: usize = 0;

        let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
            proto_packet::io::Encoder::new(&self.one, false);
        encoded_len += encoder.encode_to_write(w)?;

        let encoder: proto_packet::io::Encoder<Vec<String>> =
            proto_packet::io::Encoder::new(&self.two, false);
        encoded_len += encoder.encode_to_write(w)?;

        Ok(encoded_len)
    }
}

impl enc::DecodeFromRead for Specials {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, enc::Error>
    where
        R: std::io::Read,
    {
        use proto_packet::io::Decoder;

        let decoded_one: Vec<uuid::Uuid> = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_uuid_list(proto_packet::io::WireType::List, r, first)?
        };

        let decoded_two: Vec<String> = {
            let decoder: Decoder = Decoder::default();
            let first: u8 = enc::read_single_byte(r)?;
            decoder.decode_string_list(proto_packet::io::WireType::List, r, first)?
        };

        debug_assert!(enc::read_optional_byte(r)?.is_none());

        Ok(Self {
            one: decoded_one,
            two: decoded_two,
        })
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Specials);
