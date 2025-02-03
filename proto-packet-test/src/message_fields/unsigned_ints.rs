use proto_packet::io::WireType;
use proto_packet::{Message, Packet};

/// // A message with unsigned integers.
/// message UnsignedInts {
///   
///   // The first field.
///   one: u8 = 1;
///   
///   // The second field.
///   two: u16 = 2;
///   
///   // The third field.
///   three: u32 = 3;
///   
///   // The fourth field.
///   four: u64 = 4;
///   
///   // The fifth field.
///   five: u128 = 5;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct UnsignedInts {
    one: Option<u8>,
    two: Option<u16>,
    three: Option<u32>,
    four: Option<u64>,
    five: Option<u128>,
}

impl Packet for UnsignedInts {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for UnsignedInts {}

impl UnsignedInts {
    //! Field `one`
    //!
    //! // The first field.
    //! one: u8 = 1;

    /// Gets the field: `one`.
    pub fn one(&self) -> Option<u8> {
        self.one
    }

    /// Sets the field: `one`. Returns the previous value.
    pub fn set_one<O>(&mut self, one: O) -> Option<u8>
    where
        O: Into<Option<u8>>,
    {
        let old_one: Option<u8> = self.one;
        self.one = one.into();
        old_one
    }

    /// Builds the field: `one`. Returns the struct itself.
    pub fn with_one<O>(mut self, one: O) -> Self
    where
        O: Into<Option<u8>>,
    {
        self.one = one.into();
        self
    }
}

impl UnsignedInts {
    //! Field `two`
    //!
    //! // The second field.
    //! two: u16 = 2;

    /// Gets the field: `two`.
    pub fn two(&self) -> Option<u16> {
        self.two
    }

    /// Sets the field: `two`. Returns the previous value.
    pub fn set_two<O>(&mut self, two: O) -> Option<u16>
    where
        O: Into<Option<u16>>,
    {
        let old_two: Option<u16> = self.two;
        self.two = two.into();
        old_two
    }

    /// Builds the field: `two`. Returns the struct itself.
    pub fn with_two<O>(mut self, two: O) -> Self
    where
        O: Into<Option<u16>>,
    {
        self.two = two.into();
        self
    }
}

impl UnsignedInts {
    //! Field `three`
    //!
    //! // The third field.
    //! three: u32 = 3;

    /// Gets the field: `three`.
    pub fn three(&self) -> Option<u32> {
        self.three
    }

    /// Sets the field: `three`. Returns the previous value.
    pub fn set_three<O>(&mut self, three: O) -> Option<u32>
    where
        O: Into<Option<u32>>,
    {
        let old_three: Option<u32> = self.three;
        self.three = three.into();
        old_three
    }

    /// Builds the field: `three`. Returns the struct itself.
    pub fn with_three<O>(mut self, three: O) -> Self
    where
        O: Into<Option<u32>>,
    {
        self.three = three.into();
        self
    }
}

impl UnsignedInts {
    //! Field `four`
    //!
    //! // The fourth field.
    //! four: u64 = 4;

    /// Gets the field: `four`.
    pub fn four(&self) -> Option<u64> {
        self.four
    }

    /// Sets the field: `four`. Returns the previous value.
    pub fn set_four<O>(&mut self, four: O) -> Option<u64>
    where
        O: Into<Option<u64>>,
    {
        let old_four: Option<u64> = self.four;
        self.four = four.into();
        old_four
    }

    /// Builds the field: `four`. Returns the struct itself.
    pub fn with_four<O>(mut self, four: O) -> Self
    where
        O: Into<Option<u64>>,
    {
        self.four = four.into();
        self
    }
}

impl UnsignedInts {
    //! Field `five`
    //!
    //! // The fifth field.
    //! five: u128 = 5;

    /// Gets the field: `five`.
    pub fn five(&self) -> Option<u128> {
        self.five
    }

    /// Sets the field: `five`. Returns the previous value.
    pub fn set_five<O>(&mut self, five: O) -> Option<u128>
    where
        O: Into<Option<u128>>,
    {
        let old_five: Option<u128> = self.five;
        self.five = five.into();
        old_five
    }

    /// Builds the field: `five`. Returns the struct itself.
    pub fn with_five<O>(mut self, five: O) -> Self
    where
        O: Into<Option<u128>>,
    {
        self.five = five.into();
        self
    }
}
