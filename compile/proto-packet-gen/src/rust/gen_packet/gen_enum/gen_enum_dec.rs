use crate::rust::{GenRust, Naming};
use code_gen::rust::{
    Access, Enum as RustEnum, EnumCase as RustEnumCase, EnumFields, WithAccess,
    WithComments as RustWithComments,
};
use proto_packet_tree::{Enum, WithCaseName, WithComments, WithTagNumber};

impl GenRust {
    //! Gen Enum: Declaration

    /// Generates the enum declaration for the enum `e`.
    pub(in crate::rust::gen_packet::gen_enum) fn gen_enum_dec(&self, e: &Enum) -> RustEnum {
        let mut result: RustEnum = RustEnum::from(self.naming.type_name(e));

        for comment in e.comments() {
            result.add_comment(comment);
        }
        self.gen_derives_type_dec(&mut result, true, false);
        result.set_access(Access::Public);

        result.add_case(
            RustEnumCase::from(Naming::UNRECOGNIZED_CASE_NAME).with_fields(EnumFields::Unnamed(
                vec![format!(
                    "{}{}",
                    self.naming.type_name(e),
                    Naming::UNRECOGNIZED_CASE_NAME
                )
                .into()],
            )),
        );
        for case in e.cases() {
            let mut r: RustEnumCase = RustEnumCase::from(self.naming.case_name(case));
            for comment in case.comments() {
                r.add_comment(format!(" //{}", comment));
            }
            r.add_comment(format!(" {} = {};", case.case_name(), case.tag_number()));
            result.add_case(r)
        }

        result
    }
}
