use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet_tree::{ModPathRef, Variant};

impl GenRust {
    //! Gen Variant

    pub(in crate::rust) fn gen_variant(&self, _mod_path: ModPathRef, v: &Variant) -> Source {
        Source::default()
            .with_statement(self.gen_variant_dec(v))
            .with_statement(self.gen_variant_impls(v))
            .with_statement(self.gen_variant_encode(v))
            .with_statement(self.gen_variant_decode(v))
            .with_statement(self.gen_variant_unrecognized(v))
    }
}
