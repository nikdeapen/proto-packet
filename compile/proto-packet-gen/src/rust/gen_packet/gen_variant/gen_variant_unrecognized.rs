use crate::rust::GenRust;
use code_gen::rust::{Access, Struct, WithAccess, WithComments, WithDerives, WithStructFields};
use code_gen::{Source, WithStatements};
use proto_packet_tree::Variant;

impl GenRust {
    //! Gen Variant: Unrecognized

    pub(in crate::rust::gen_packet::gen_variant) fn gen_variant_unrecognized(
        &self,
        v: &Variant,
    ) -> Source {
        Source::default()
            .with_empty_line()
            .with_statement(self.gen_variant_unrecognized_dec(v))
    }

    fn gen_variant_unrecognized_dec(&self, v: &Variant) -> Struct {
        Struct::from(self.naming.unrecognized_case_type_name(v))
            .with_comment(format!(
                " An unrecognized `{}` case.",
                self.naming.type_name(v)
            ))
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
            .with_field(("serial", "Vec<u8>"))
    }
}
