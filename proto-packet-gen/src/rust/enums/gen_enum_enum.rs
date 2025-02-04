use code_gen::rust::Access::Public;
use code_gen::rust::{
    Enum as RustEnum, EnumCase as RustEnumCase, EnumFields, RustType, WithAccess,
    WithComments as RustComments, WithDerives,
};

use proto_packet_tree::{Enum, WithCaseName, WithComments, WithTagNumber, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Enum Enum

    pub(in crate::rust::enums) fn gen_enum_enum(&self, enom: &Enum) -> RustEnum {
        let mut e: RustEnum = RustEnum::from(self.naming.type_name(enom.type_name()));

        self.gen_enum_comments(&mut e, enom);
        e = e
            .with_derive("Copy")
            .with_derive("Clone")
            .with_derive("Ord")
            .with_derive("PartialOrd")
            .with_derive("Eq")
            .with_derive("PartialEq")
            .with_derive("Hash")
            .with_derive("Debug");
        e.set_access(Public);

        self.gen_enum_enum_cases(&mut e, enom);

        e
    }

    fn gen_enum_comments(&self, e: &mut RustEnum, enom: &Enum) {
        for comment in enom.comments() {
            e.add_comment(format!("{}", comment));
        }
    }

    fn gen_enum_enum_cases(&self, e: &mut RustEnum, enom: &Enum) {
        e.add_case(
            RustEnumCase::from("Unrecognized")
                .with_comment(" An enum case with an unrecognized tag number.")
                .with_fields(EnumFields::Unnamed(vec![RustType::from(
                    "proto_packet::io::TagNumber",
                )])),
        );

        for case in enom.cases() {
            let mut c: RustEnumCase = RustEnumCase::from(self.naming.case_name(case.case_name()));
            for comment in case.comments() {
                c.add_comment(format!(" //{}", comment));
            }
            c.add_comment(format!(" {} = {};", case.case_name(), case.tag_number()));
            e.add_case(c);
        }
    }
}
