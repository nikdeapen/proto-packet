use code_gen::{Source, WithStatements};

use proto_packet_tree::Variant;

use crate::rust::GenRust;

impl GenRust {
    //! Gen Variant

    /// Generates the code for the `variant`.
    pub fn gen_variant(&self, variant: &Variant) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(self.gen_variant_imports());
        source.add_statement(self.gen_variant_enum(variant));
        source.add_statement(self.gen_variant_tag_number(variant));
        source.add_statement(self.gen_variant_impls(variant));
        source.add_statement(self.gen_variant_encode(variant));
        source.add_statement(self.gen_variant_decode(variant));

        source
    }
}
