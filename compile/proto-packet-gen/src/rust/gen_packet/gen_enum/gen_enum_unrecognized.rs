use crate::rust::{GenRust, Naming};
use code_gen::rust::{Access, Struct, WithAccess, WithComments, WithDerives, WithStructFields};
use code_gen::{Source, WithStatements};
use proto_packet_tree::Enum;

impl GenRust {
    //! Gen Enum: Unrecognized

    pub(in crate::rust::gen_packet::gen_enum) fn gen_enum_unrecognized(&self, e: &Enum) -> Source {
        Source::default()
            .with_empty_line()
            .with_statement(self.gen_enum_unrecognized_dec(e))
    }

    fn gen_enum_unrecognized_dec(&self, e: &Enum) -> Struct {
        Struct::from(format!(
            "{}{}",
            self.naming.type_name(e),
            Naming::UNRECOGNIZED_ENUM_CASE_NAME
        ))
        .with_comment(format!(
            " An unrecognized `{}` case.",
            self.naming.type_name(e)
        ))
        .with_derive("Copy")
        .with_derive("Clone")
        .with_derive("Ord")
        .with_derive("PartialOrd")
        .with_derive("Eq")
        .with_derive("PartialEq")
        .with_derive("Hash")
        .with_derive("Debug")
        .with_derive("serde::Serialize")
        .with_derive("serde::Deserialize")
        .with_access(Access::Public)
        .with_field(("tag", "proto_packet::io::TagNumber"))
    }
}
