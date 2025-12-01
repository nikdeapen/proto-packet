use crate::rust::GenRust;
use code_gen::rust::Access::Public;
use code_gen::rust::{
    RustType, Struct as RustStruct, StructField as RustStructField, WithAccess, WithComments,
    WithStructFields,
};
use proto_packet_tree::{Struct, StructField, WithFieldName, WithTypeTag};

impl GenRust {
    //! Gen Struct: Type Dec

    /// Generates the struct declaration for the struct `s`.
    pub(in crate::rust) fn gen_struct_type_dec(&self, s: &Struct) -> RustStruct {
        let mut result: RustStruct = RustStruct::from(self.naming.type_name(s));

        self.gen_struct_type_dec_comments(&mut result, s);
        self.gen_derives_type_dec(
            &mut result,
            s.fields().iter().all(|f| self.typing.is_copy(f.type_tag())),
            false,
        );
        result.set_access(Public);

        for field in s.fields() {
            result.add_field(self.gen_struct_type_dec_field(field));
        }

        result
    }

    /// Generates the comments for the `result` struct declaration for the struct `s`.
    fn gen_struct_type_dec_comments(&self, result: &mut RustStruct, s: &Struct) {
        self.gen_comments_type_dec(result, s, "struct");
        for field in s.fields() {
            self.gen_comments_field(result, field, field.field_name(), field.type_tag(), None);
        }
        result.add_comment(" }");
    }

    /// Generates the struct field for the struct `field`.
    fn gen_struct_type_dec_field(&self, field: &StructField) -> RustStructField {
        let field_name: String = self.naming.field_name(field);
        let type_tag: RustType = self.typing.field_type(field.type_tag(), false);
        RustStructField::from((field_name, type_tag))
    }
}
