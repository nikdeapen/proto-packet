use proto_packet::io::WireType;
use proto_packet::{Message, Packet};

/// // A message with special types.
/// message SpecialTypes {
///   
///   // The first field.
///   one: uuid = 1;
///   
///   // The second field.
///   two: string = 2;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct SpecialTypes {
    one: Option<uuid::Uuid>,
    two: Option<String>,
}

impl Packet for SpecialTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for SpecialTypes {}

impl SpecialTypes {
    //! Field `one`
    //!
    //! // The first field.
    //! one: uuid = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<uuid::Uuid> {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<O>(&mut self, one: O) -> Option<uuid::Uuid>
    where
        O: Into<Option<uuid::Uuid>>,
    {
        let old_one: Option<uuid::Uuid> = self.one;
        self.one = one.into();
        old_one
    }

    /// Builds the field: `one`. Returns the struct itself.
    pub fn with_one<O>(mut self, one: O) -> Self
    where
        O: Into<Option<uuid::Uuid>>,
    {
        self.one = one.into();
        self
    }
}

impl SpecialTypes {
    //! Field `two`
    //!
    //! // The second field.
    //! two: string = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<&str> {
        self.two.as_deref()
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<O>(&mut self, two: O) -> Option<String>
    where
        O: Into<Option<String>>,
    {
        let two: Option<String> = two.into();
        std::mem::replace(&mut self.two, two)
    }

    /// Builds the field: `two`. Returns the struct itself.
    pub fn with_two<O>(mut self, two: O) -> Self
    where
        O: Into<Option<String>>,
    {
        self.two = two.into();
        self
    }
}
