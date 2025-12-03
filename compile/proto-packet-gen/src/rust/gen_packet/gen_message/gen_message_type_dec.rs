use crate::rust::GenRust;
use code_gen::rust::Access::Public;
use code_gen::rust::{RustType, Struct, StructField, WithAccess, WithComments, WithStructFields};
use proto_packet_tree::{Message, MessageField, WithFieldName, WithTypeTag};

impl GenRust {
    //! Gen Message: Type Dec

    /// Generates the struct declaration for the message `m`.
    pub(in crate::rust::gen_packet::gen_message) fn gen_message_type_dec(
        &self,
        m: &Message,
    ) -> Struct {
        let mut result: Struct = Struct::from(self.naming.type_name(m));

        self.gen_message_type_dec_comments(&mut result, m);
        self.gen_derives_type_dec(&mut result, false, true);
        result.set_access(Public);

        for field in m.fields() {
            result.add_field(self.gen_message_type_dec_field(field));
        }

        result
    }

    /// Generates the comments for the `result` struct declaration for the message `m`.
    fn gen_message_type_dec_comments(&self, result: &mut Struct, m: &Message) {
        self.gen_comments_type_dec(result, m, "struct");
        for field in m.fields() {
            self.gen_comments_field(result, field, field.field_name(), field.type_tag(), None);
        }
        result.add_comment(" }");
    }

    /// Generates the struct field for the message `field`.
    fn gen_message_type_dec_field(&self, field: &MessageField) -> StructField {
        let field_name: String = self.naming.field_name(field);
        let type_tag: RustType = self.typing.field_type(field.type_tag(), true);
        StructField::from((field_name, type_tag))
    }
}
