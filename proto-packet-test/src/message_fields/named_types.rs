use proto_packet::io::WireType;
use proto_packet::{Message, Packet};

/// // A message with named types.
/// message NamedTypes {
///   
///   // A local message field.
///   local_message: message_fields.UnsignedInts = 1;
/// }
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct NamedTypes {
    local_message: Option<crate::message_fields::UnsignedInts>,
}

impl Packet for NamedTypes {
    fn wire_type() -> WireType {
        WireType::LengthPrefixed
    }
}

impl Message for NamedTypes {}

impl NamedTypes {
    //! Field `local_message`
    //!
    //! // A local message field.
    //! local_message: message_fields.UnsignedInts = 1;

    /// Gets the field: `local_message`.
    pub fn local_message(&self) -> Option<&crate::message_fields::UnsignedInts> {
        self.local_message.as_ref()
    }

    /// Sets the field: `local_message`. Returns the previous value.
    pub fn set_local_message<O>(
        &mut self,
        local_message: O,
    ) -> Option<crate::message_fields::UnsignedInts>
    where
        O: Into<Option<crate::message_fields::UnsignedInts>>,
    {
        let local_message: Option<crate::message_fields::UnsignedInts> = local_message.into();
        std::mem::replace(&mut self.local_message, local_message)
    }

    /// Builds the field: `local_message`. Returns the struct itself.
    pub fn with_local_message<O>(mut self, local_message: O) -> Self
    where
        O: Into<Option<crate::message_fields::UnsignedInts>>,
    {
        self.local_message = local_message.into();
        self
    }
}
