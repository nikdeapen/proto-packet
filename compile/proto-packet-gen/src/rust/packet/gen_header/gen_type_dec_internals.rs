use crate::rust::GenRust;
use code_gen::rust::{
    EnumCase as RustEnumCase, EnumFields, RustType, StructField as RustStructField,
    WithComments as RustWithComments,
};
use proto_packet_tree::{
    EnumCase, MessageField, StructField, VariantCase, WithCaseName, WithComments, WithFieldName,
    WithTagNumber, WithTypeTag,
};

impl GenRust {
    //! Gen Dec: Internals

    /// Generates the struct field for the struct `field`.
    pub(in crate::rust) fn gen_type_dec_struct_field(
        &self,
        field: &StructField,
    ) -> RustStructField {
        let field_name: String = self.naming.field_name(field.field_name());
        let type_tag: RustType = self.typing.field_type(field.type_tag(), false);

        RustStructField::from((field_name, type_tag))
    }

    /// Generates the struct field for the message `field`.
    pub(in crate::rust) fn gen_type_dec_message_field(
        &self,
        field: &MessageField,
    ) -> RustStructField {
        let field_name: String = self.naming.field_name(field.field_name());
        let type_tag: RustType = self.typing.field_type(field.type_tag(), true);

        RustStructField::from((field_name, type_tag))
    }

    /// Generates the enum case for the enum `case`.
    pub(in crate::rust) fn gen_type_dec_enum_case(&self, case: &EnumCase) -> RustEnumCase {
        let mut result: RustEnumCase = RustEnumCase::from(self.naming.case_name(case.case_name()));

        for comment in case.comments() {
            result.add_comment(format!(" //{}", comment));
        }
        result.add_comment(format!(" {} = {};", case.case_name(), case.tag_number()));

        result
    }

    /// Generates the enum case for the variant `case`.
    pub(in crate::rust) fn gen_type_dec_variant_case(&self, case: &VariantCase) -> RustEnumCase {
        let mut result: RustEnumCase = RustEnumCase::from(self.naming.case_name(case.case_name()));

        for comment in case.comments() {
            result.add_comment(format!(" //{}", comment));
        }
        result.add_comment(format!(
            " {}: {} = {};",
            case.case_name(),
            case.type_tag(),
            case.tag_number()
        ));

        result.set_fields(EnumFields::Unnamed(vec![self
            .typing
            .field_type(case.type_tag(), false)]));

        result
    }
}
