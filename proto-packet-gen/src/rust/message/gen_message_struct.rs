use code_gen::rust::Access::Public;
use code_gen::rust::{
    RustType, Struct, StructField, WithAccess, WithComments as RustWithComments, WithDerives,
    WithStructFields,
};

use proto_packet_tree::{
    Message, MessageField, WithComments, WithFieldName, WithTagNumberOptional, WithTypeName,
    WithTypeTag,
};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Message Struct

    /// Generates the `struct` declaration for the `message`.
    pub(in crate::rust::message) fn gen_message_struct(&self, message: &Message) -> Struct {
        let mut s: Struct = Struct::from(self.naming.type_name(message.type_name()));

        self.gen_struct_comments(&mut s, message);
        s = s
            .with_derive("Clone")
            .with_derive("Ord")
            .with_derive("PartialOrd")
            .with_derive("Eq")
            .with_derive("PartialEq")
            .with_derive("Hash")
            .with_derive("Debug")
            .with_derive("Default");
        s.set_access(Public);

        for field in message.fields() {
            s.add_field(self.gen_struct_field(field));
        }

        s
    }

    fn gen_struct_comments(&self, s: &mut Struct, message: &Message) {
        for comment in message.comments() {
            s.add_comment(format!(" //{}", comment));
        }
        s.add_comment(format!(" message {} {{", message.type_name()));
        for field in message.fields() {
            for comment in field.comments() {
                s.add_comment("   ");
                s.add_comment(format!("   //{}", comment));
            }
            if let Some(tag_number) = field.tag_number() {
                s.add_comment(format!(
                    "   {}: {} = {};",
                    field.field_name(),
                    field.type_tag(),
                    tag_number
                ));
            } else {
                s.add_comment(format!("   {}: {};", field.field_name(), field.type_tag()));
            }
        }
        s.add_comment(" }");
    }

    fn gen_struct_field(&self, field: &MessageField) -> StructField {
        let name: String = self.naming.field_name(field.field_name());
        let mut type_tag: RustType = self.typing.field_type(field.type_tag());
        if field.tag_number().is_some() {
            type_tag = type_tag.to_option();
        }
        StructField::from((name, type_tag))
    }
}
