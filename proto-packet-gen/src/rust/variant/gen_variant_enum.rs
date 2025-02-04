use code_gen::rust::Access::Public;
use code_gen::rust::{
    Enum, EnumCase, EnumFields, WithAccess, WithComments as RustComments, WithDerives,
};

use proto_packet_tree::{Variant, WithCaseName, WithComments, WithTypeName, WithTypeTag};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Variant Enum

    /// Generates the `enum` declaration for the `variant`.
    pub fn gen_variant_enum(&self, variant: &Variant) -> Enum {
        let mut enom: Enum = Enum::from(self.naming.type_name(variant.type_name()));

        self.gen_variant_comments(&mut enom, variant);
        enom = enom
            .with_derive("Clone")
            .with_derive("Ord")
            .with_derive("PartialOrd")
            .with_derive("Eq")
            .with_derive("PartialEq")
            .with_derive("Hash")
            .with_derive("Debug");
        enom.set_access(Public);

        self.gen_variant_enum_cases(&mut enom, variant);

        enom
    }

    fn gen_variant_comments(&self, enom: &mut Enum, variant: &Variant) {
        for comment in variant.comments() {
            enom.add_comment(comment);
        }
    }

    fn gen_variant_enum_cases(&self, enom: &mut Enum, variant: &Variant) {
        for case in variant.cases() {
            let mut c: EnumCase = EnumCase::from(self.naming.case_name(case.case_name()))
                .with_fields(EnumFields::Unnamed(vec![self
                    .typing
                    .field_type(case.type_tag())]));
            for comment in case.comments() {
                c.add_comment(format!(" //{}", comment));
            }
            c.add_comment(format!(" {} = {};", case.case_name(), case.tag_number()));
            enom.add_case(c);
        }
    }
}
