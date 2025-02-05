use code_gen::{Source, WithStatements};

use proto_packet_tree::Enum;

use crate::rust::GenRust;

impl GenRust {
    //! Gen Enum

    /// Generates the code for the `enum`.
    pub fn gen_enum(&self, enom: &Enum) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(self.gen_enum_imports());
        source.add_statement(self.gen_enum_enum(enom));
        source.add_statement(self.gen_enum_from_str(enom));
        source.add_statement(self.gen_enum_tag_number(enom));
        source.add_statement(self.gen_enum_impls(enom));
        source.add_statement(self.gen_enum_encode(enom));
        source.add_statement(self.gen_enum_decode(enom));

        source
    }
}
