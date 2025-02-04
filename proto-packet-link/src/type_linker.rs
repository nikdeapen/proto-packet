use std::collections::HashMap;

use proto_packet_tree::{
    Import, Message, MessageField, ModPathRef, QualifiedName, QualifiedNameRef, TypeDec,
    TypeNameRef, TypeTag, Variant, VariantCase, WithCaseName, WithComments, WithFieldName,
    WithTagNumberOptional, WithTypeName, WithTypeTag,
};
use proto_packet_tree::TypeDec::*;

use crate::Error;
use crate::Error::*;

/// Responsible for linking types.
#[derive(Debug)]
pub struct TypeLinker<'a> {
    mod_path: ModPathRef<'a>,
    local_names: &'a [TypeNameRef<'a>],
    imports: &'a [Import],
    all_names: &'a HashMap<ModPathRef<'a>, Vec<TypeNameRef<'a>>>,
}

impl<'a> TypeLinker<'a> {
    //! Construction

    /// Creates a new type tag linker.
    ///
    /// # Unsafe
    /// -- todo
    pub unsafe fn new(
        mod_path: ModPathRef<'a>,
        local_names: &'a [TypeNameRef<'a>],
        imports: &'a [Import],
        all_names: &'a HashMap<ModPathRef<'a>, Vec<TypeNameRef<'a>>>,
    ) -> Self {
        Self {
            mod_path,
            local_names,
            imports,
            all_names,
        }
    }
}

impl<'a> TypeLinker<'a> {
    //! Link

    /// Links the `type_dec`.
    pub fn link(&self, type_dec: &TypeDec) -> Result<TypeDec, Error> {
        Ok(match type_dec {
            MessageDec(message) => MessageDec(self.link_message(message)?),
            EnumDec(enom) => EnumDec(enom.clone()),
            VariantDec(variant) => VariantDec(self.link_variant(variant)?),
        })
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
        let mut linked: MessageField = MessageField::new(field.field_name(), type_tag);
        for comment in field.comments() {
            linked.add_comment(comment);
        }
        linked.set_tag_number(field.tag_number());
        Ok(linked)
    }
}

impl<'a> TypeLinker<'a> {
    //! Variants

    /// Links the `variant`.
    fn link_variant(&self, variant: &Variant) -> Result<Variant, Error> {
        let mut linked: Variant = variant.type_name().into();
        for comment in variant.comments() {
            linked.add_comment(comment);
        }
        for case in variant.cases() {
            let linked_case: VariantCase = self.link_variant_case(case)?;
            debug_assert!(linked.can_add_case(&linked_case));
            unsafe { linked.add_case(linked_case) };
        }
        Ok(linked)
    }

    /// Links the variant `case`.
    fn link_variant_case(&self, case: &VariantCase) -> Result<VariantCase, Error> {
        let type_tag: TypeTag = self.link_type_tag(case.type_tag())?;
        let mut linked: VariantCase =
            VariantCase::new(case.case_name(), type_tag, case.tag_number());
        for comment in case.comments() {
            linked.add_comment(comment);
        }
        Ok(linked)
    }
}

impl<'a> TypeLinker<'a> {
    //! Type Tags

    /// Links the `type_tag`.
    fn link_type_tag(&self, type_tag: &TypeTag) -> Result<TypeTag, Error> {
        match type_tag {
            TypeTag::Primitive(primitive) => Ok(primitive.to_type_tag()),
            TypeTag::Special(special) => Ok(special.to_type_tag()),
            TypeTag::Named(name) => Ok(self.resolve_name(name.to_ref())?.into()),
        }
    }

    /// Resolves the `name`.
    fn resolve_name(&self, name: QualifiedNameRef) -> Result<QualifiedName, Error> {
        if let Some(mod_path) = name.mod_path() {
            if let Some(type_names) = self.all_names.get(&mod_path) {
                for type_name in type_names {
                    if name.type_name() == &type_name {
                        return Ok(name.to_owned());
                    }
                }
            }
        } else {
            for local_name in self.local_names {
                if name.type_name() == local_name {
                    return Ok(self.mod_path.to_qualified_name(name.type_name()));
                }
            }
            for import in self.imports {
                if name.type_name() == import.effective_name() {
                    return Ok(import.name().to_owned());
                }
            }
        }
        Err(UnrecognizedName {
            context: self.mod_path.to_owned(),
            name: name.to_owned(),
        })
    }
}
