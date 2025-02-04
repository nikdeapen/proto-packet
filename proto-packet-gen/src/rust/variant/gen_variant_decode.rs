use code_gen::rust::ImplBlock;
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Variant, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Variant Decode

    /// Generates the decoding trait implementations for the `variant`.
    pub fn gen_variant_decode(&self, variant: &Variant) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_decode_from_read(variant));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_decode_from_read_prefix(variant));

        source
    }
}

impl GenRust {
    //! `enc::DecodeFromRead`

    pub fn gen_variant_decode_from_read(&self, variant: &Variant) -> ImplBlock {
        let source: Source = Source::default()
            .with_semi("use enc::DecodeFromReadPrefix")
            .with_literal("Self::decode_from_read_prefix(r)");
        self.gen_decode_from_read_impl(variant.type_name(), source)
    }
}

impl GenRust {
    //! `enc::DecodeFromReadPrefix`

    pub fn gen_variant_decode_from_read_prefix(&self, variant: &Variant) -> ImplBlock {
        self.gen_decode_from_read_prefix_impl(
            variant.type_name(),
            Source::default()
                .with_semi("use enc::DecodeFromRead")
                .with_literal("Self::decode_from_read_length_prefixed_with_first_byte(first, r)"),
        )
    }
}
