use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet::PacketType;
use proto_packet_tree::{Enum, Message, ModPathRef, Struct, Variant};

impl GenRust {
    //! Gen Packet

    /// Generates the source code for the struct `s`.
    pub(in crate::rust) fn gen_struct(&self, mod_path: ModPathRef, s: &Struct) -> Source {
        Source::default()
            .with_statement(self.gen_imports(PacketType::Struct))
            .with_statement(self.gen_type_dec_struct(s))
            .with_statement(self.gen_struct_constructors(s))
            .with_statement(self.gen_fields_struct(s))
            .with_statement(self.gen_type_impls_struct(s))
            .with_statement(self.gen_encode_struct(s))
            .with_statement(self.gen_decode_struct(mod_path, s))
    }

    /// Generates the source code for the message `m`.
    pub(in crate::rust) fn gen_message(&self, mod_path: ModPathRef, m: &Message) -> Source {
        Source::default()
            .with_statement(self.gen_imports(PacketType::Message))
            .with_statement(self.gen_type_dec_message(m))
            .with_statement(self.gen_fields_message(m))
            .with_statement(self.gen_type_impls_message(m))
            .with_statement(self.gen_encode_message(m))
            .with_statement(self.gen_decode_message(mod_path, m))
    }

    /// Generates the source code for the enum `e`.
    pub(in crate::rust) fn gen_enum(&self, mod_path: ModPathRef, e: &Enum) -> Source {
        Source::default()
            .with_statement(self.gen_imports(PacketType::Enum))
            .with_statement(self.gen_type_dec_enum(e))
            .with_statement(self.gen_type_impls_enum(e))
            .with_statement(self.gen_encode_enum(e))
            .with_statement(self.gen_decode_enum(mod_path, e))
    }

    /// Generates the source code for the variant `v`.
    pub(in crate::rust) fn gen_variant(&self, mod_path: ModPathRef, v: &Variant) -> Source {
        Source::default()
            .with_statement(self.gen_imports(PacketType::Variant))
            .with_statement(self.gen_type_dec_variant(v))
            .with_statement(self.gen_type_impls_variant(v))
            .with_statement(self.gen_encode_variant(v))
            .with_statement(self.gen_decode_variant(mod_path, v))
    }
}
