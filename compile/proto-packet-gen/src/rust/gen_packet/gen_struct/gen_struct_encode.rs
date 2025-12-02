use crate::rust::{EncodeOp, GenRust};
use code_gen::{Source, WithStatements};
use proto_packet_tree::{Struct, WithTypeTag};

impl GenRust {
    //! Gen Struct: Encode

    pub(in crate::rust::gen_packet::gen_struct) fn gen_struct_encode(&self, s: &Struct) -> Source {
        self.gen_encode(s, false, |s, op| self.gen_struct_encode_source(s, op))
    }

    fn gen_struct_encode_source(&self, s: &Struct, op: EncodeOp) -> Source {
        let mut source: Source = Source::default();
        source.add_semi("let mut encoded_len: usize = 0");

        for field in s.fields() {
            source.add_empty_line();
            source.add_statement(self.gen_encode_value(
                format!("&self.{}", self.naming.field_name(field)).as_str(),
                field.type_tag(),
                false, // todo -- fixed
                op,
            ));
        }

        source.add_empty_line();
        source.add_literal("Ok(encoded_len)");
        source
    }
}
