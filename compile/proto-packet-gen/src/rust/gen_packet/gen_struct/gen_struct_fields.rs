use crate::rust::GenRust;
use code_gen::rust::WithFunctions;
use code_gen::{Source, WithStatements};
use proto_packet_tree::Struct;

impl GenRust {
    //! Gen Struct: Fields

    /// Generates the field impl blocks for the struct `s`.
    pub(in crate::rust::gen_packet::gen_struct) fn gen_struct_fields(&self, s: &Struct) -> Source {
        let mut source: Source = Source::default();

        for field in s.fields() {
            source.add_empty_line();
            source.add_statement(
                self.gen_field_impl_block(s, field, None)
                    .with_function(self.gen_field_getter(field, false))
                    .with_function(self.gen_field_setter(field, false))
                    .with_function(self.gen_field_builder(field, false)),
            );
        }

        source
    }
}
