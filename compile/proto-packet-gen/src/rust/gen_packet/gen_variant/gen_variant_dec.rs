use crate::rust::{GenRust, Naming};
use code_gen::rust::EnumFields::Unnamed;
use code_gen::rust::{
    Access, Enum, EnumCase, EnumFields, WithAccess, WithComments as RustWithComments,
};
use proto_packet_tree::{Variant, WithComments, WithTypeTag};

impl GenRust {
    //! Gen Variant: Declaration

    pub(in crate::rust::gen_packet::gen_variant) fn gen_variant_dec(&self, v: &Variant) -> Enum {
        let mut result: Enum = Enum::from(self.naming.type_name(v));
        for comment in v.comments() {
            result.add_comment(comment);
        }
        self.gen_derives_type_dec(&mut result, false, false);
        result.set_access(Access::Public);

        result.add_case(EnumCase::from(Naming::UNRECOGNIZED_CASE_NAME).with_fields(
            EnumFields::Unnamed(vec![self.naming.unrecognized_case_type_name(v).into()]),
        ));
        for case in v.cases() {
            result.add_case(
                EnumCase::from(self.naming.case_name(case))
                    .with_fields(Unnamed(vec![self.typing.owned(case.type_tag())])),
            );
        }

        result
    }
}
