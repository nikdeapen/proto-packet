use crate::{Error, Resolver};
use proto_packet_tree::TypeDec::*;
use proto_packet_tree::{
    Message, MessageField, Struct, StructField, TypeDec, TypeTag, WithComments, WithFieldName,
    WithTagNumber, WithTypeName, WithTypeTag,
};

/// Responsible for linking types.
#[derive(Debug)]
pub struct TypeLinker<'a> {
    resolver: Resolver<'a>,
}

impl<'a> From<Resolver<'a>> for TypeLinker<'a> {
    fn from(resolver: Resolver<'a>) -> Self {
        Self { resolver }
    }
}

impl<'a> TypeLinker<'a> {
    //! Link

    /// Links the `type_dec`.
    pub fn link(&self, type_dec: &TypeDec) -> Result<TypeDec, Error> {
        Ok(match type_dec {
            StructDec(s) => StructDec(self.link_struct(s)?),
            MessageDec(m) => MessageDec(self.link_message(m)?),
        })
    }
}

impl<'a> TypeLinker<'a> {
    //! Structs

    /// Links the `structure`.
    fn link_struct(&self, structure: &Struct) -> Result<Struct, Error> {
        let mut linked: Struct = structure.type_name().into();
        for comment in structure.comments() {
            linked.add_comment(comment);
        }
        for field in structure.fields() {
            let linked_field: StructField = self.link_struct_field(field)?;
            debug_assert!(linked.can_add_field(&linked_field));
            unsafe { linked.add_field(linked_field) };
        }
        Ok(linked)
    }

    /// Links the struct `field`.
    fn link_struct_field(&self, field: &StructField) -> Result<StructField, Error> {
        let type_tag: TypeTag = self.link_type_tag(field.type_tag())?;
        let mut linked: StructField = StructField::new(field.field_name(), type_tag);
        for comment in field.comments() {
            linked.add_comment(comment);
        }
        Ok(linked)
    }
}

impl<'a> TypeLinker<'a> {
    //! Messages

    /// Links the `message`.
    fn link_message(&self, message: &Message) -> Result<Message, Error> {
        let mut linked: Message = message.type_name().into();
        for comment in message.comments() {
            linked.add_comment(comment);
        }
        for field in message.fields() {
            let linked_field: MessageField = self.link_message_field(field)?;
            debug_assert!(linked.can_add_field(&linked_field));
            unsafe { linked.add_field(linked_field) };
        }
        Ok(linked)
    }

    /// Links the message `field`.
    fn link_message_field(&self, field: &MessageField) -> Result<MessageField, Error> {
        let type_tag: TypeTag = self.link_type_tag(field.type_tag())?;
        let mut linked: MessageField =
            MessageField::new(field.field_name(), type_tag, field.tag_number());
        for comment in field.comments() {
            linked.add_comment(comment);
        }
        Ok(linked)
    }
}

impl<'a> TypeLinker<'a> {
    //! Link: Type Tag

    /// Links the `type_tag`.
    fn link_type_tag(&self, type_tag: &TypeTag) -> Result<TypeTag, Error> {
        match type_tag {
            TypeTag::Primitive(primitive) => Ok(primitive.to_type_tag()),
            TypeTag::Special(special) => Ok(special.to_type_tag()),
            TypeTag::Named(name) => Ok(self.resolver.resolve(name.to_ref())?.into()),
        }
    }
}
