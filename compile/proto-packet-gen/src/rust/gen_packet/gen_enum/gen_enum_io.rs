use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet_tree::Enum;

impl GenRust {
    //! Gen Enum: IO

    pub(in crate::rust::gen_packet::gen_enum) fn gen_enum_encode(&self, e: &Enum) -> Source {
        Source::default().with_empty_line().with_semi(format!(
            "proto_packet::impl_encode_enum!({})",
            self.naming.type_name(e)
        ))
    }

    pub(in crate::rust::gen_packet::gen_enum) fn gen_enum_decode(&self, e: &Enum) -> Source {
        Source::default().with_empty_line().with_semi(format!(
            "proto_packet::impl_decode_enum!({})",
            self.naming.type_name(e)
        ))
    }
}
