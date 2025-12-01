use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet_tree::{ModPathRef, Struct};

impl GenRust {
    //! Gen Struct

    /// Generates the source code for the struct `s`.
    pub(in crate::rust) fn gen_struct(&self, _mod_path: ModPathRef, s: &Struct) -> Source {
        Source::default().with_statement(self.gen_struct_type_dec(s))
    }
}
