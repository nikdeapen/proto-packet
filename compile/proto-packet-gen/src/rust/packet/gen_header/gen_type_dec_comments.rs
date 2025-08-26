use crate::rust::GenRust;
use code_gen::rust::{Enum as RustEnum, Struct as RustStruct, WithComments as RustWithComments};
use proto_packet::io::TagNumber;
use proto_packet_tree::{
    Enum, Message, Struct, TypeTag, Variant, WithComments, WithFieldName, WithTagNumber,
    WithTypeName, WithTypeTag,
};

impl GenRust {
    //! Gen Type Dec: Comments

    /// Generates the comments for the `result` struct declaration for the struct `s`.
    pub(in crate::rust) fn gen_type_dec_comments_struct(
        &self,
        result: &mut RustStruct,
        s: &Struct,
    ) {
        for comment in s.comments() {
            result.add_comment(format!(" //{}", comment));
        }
        result.add_comment(format!(" struct {} {{", s.type_name()));
        for field in s.fields() {
            self.gen_type_dec_field_comments(
                result,
                field,
                field.field_name(),
                field.type_tag(),
                None,
            );
        }
        result.add_comment(" }");
    }

    /// Generates the comments for the `result` struct declaration for the message `m`.
    pub(in crate::rust) fn gen_type_dec_comments_message(
        &self,
        result: &mut RustStruct,
        m: &Message,
    ) {
        for comment in m.comments() {
            result.add_comment(format!(" //{}", comment));
        }
        result.add_comment(format!(" message {} {{", m.type_name()));
        for field in m.fields() {
            self.gen_type_dec_field_comments(
                result,
                field,
                field.field_name(),
                field.type_tag(),
                Some(field.tag_number()),
            );
        }
        result.add_comment(" }");
    }

    /// Generates the comments for the `result` enum declaration for the enum `e`.
    pub(in crate::rust) fn gen_type_dec_comments_enum(&self, result: &mut RustEnum, e: &Enum) {
        for comment in e.comments() {
            result.add_comment(comment);
        }
    }

    /// Generates the comments for the `result` enum declaration for the variant `v`.
    pub(in crate::rust) fn gen_type_dec_comments_variant(
        &self,
        result: &mut RustEnum,
        v: &Variant,
    ) {
        for comment in v.comments() {
            result.add_comment(comment);
        }
    }

    /// Generates the comments for struct or message field and adds them to the `result`.
    fn gen_type_dec_field_comments<T, C, N>(
        &self,
        result: &mut T,
        field_comments: &C,
        field_name: N,
        field_type: &TypeTag,
        tag_number: Option<TagNumber>,
    ) where
        T: RustWithComments,
        C: WithComments,
        N: AsRef<str>,
    {
        result.add_comment("    ");
        for comment in field_comments.comments() {
            result.add_comment(format!("    //{}", comment));
        }
        if let Some(tag_number) = tag_number {
            result.add_comment(format!(
                "    {}: {} = {};",
                field_name.as_ref(),
                field_type.to_string(),
                tag_number
            ))
        } else {
            result.add_comment(format!(
                "    {}: {};",
                field_name.as_ref(),
                field_type.to_string()
            ));
        }
    }
}
