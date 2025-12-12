use crate::rust::Typing;
use proto_packet_tree::{SpecialType, TypeTag};

impl Typing {
    //! Is `Copy`

    /// Checks if the `type_tag` converts to a Rust `Copy` type.
    pub fn is_copy(&self, type_tag: &TypeTag) -> bool {
        match type_tag {
            TypeTag::Primitive(_) => true,
            TypeTag::Special(special) => match special {
                SpecialType::Uuid => true,
                SpecialType::String => false,
            },
            TypeTag::Named(_) => false,
            TypeTag::List(_) => false,
        }
    }
}
