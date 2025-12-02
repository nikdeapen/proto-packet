use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet_tree::{ModPathRef, Struct, TypeTag, WithFieldName, WithTypeName, WithTypeTag};

impl GenRust {
    //! Gen Struct: Decode

    pub(in crate::rust::gen_packet::gen_struct) fn gen_struct_decode(
        &self,
        mod_path: ModPathRef,
        s: &Struct,
    ) -> Source {
        Source::default()
            .with_empty_line()
            .with_statement(self.gen_decode_from_read_impl(
                s.type_name(),
                self.gen_decode_from_read_source_struct(mod_path, s),
            ))
            .with_empty_line()
            .with_semi(format!(
                "enc::impl_decode_from_read_prefix_length_prefixed!({})",
                self.naming.type_name(s)
            ))
    }

    fn gen_decode_from_read_source_struct(&self, _mod_path: ModPathRef, s: &Struct) -> Source {
        let mut source: Source = Source::default();

        source.add_semi("use proto_packet::io::Decoder");
        if s.fields()
            .iter()
            .any(|f| matches!(f.type_tag(), TypeTag::Named(_)))
        {
            source.add_semi("use proto_packet::Packet");
        }
        source.add_empty_line();

        for field in s.fields() {
            source.add_empty_line();
            source.add_statement(self.gen_decode_value(
                format!("decoded_{}", self.naming.field_name(field)).as_str(),
                field.field_name(),
                field.type_tag(),
                false,
                None,
            ));
        }

        source.add_empty_line();
        source.add_semi("debug_assert!(enc::read_optional_byte(r)?.is_none())");

        source.add_empty_line();
        source.add_literal("Ok(Self {");
        for field in s.fields() {
            source.add_literal(format!(
                "    {}: decoded_{},",
                self.naming.field_name(field),
                self.naming.field_name(field)
            ));
        }
        source.add_literal("})");

        source
    }
}
