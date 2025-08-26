use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet::io::TagNumber;
use proto_packet_tree::TypeTag;

impl GenRust {
    //! Gen Decode Value

    pub(in crate::rust) fn gen_decode_value<S>(
        &self,
        var: &str,
        _field_name: S,
        type_tag: &TypeTag,
        fixed: bool,
        _tag_number: Option<TagNumber>,
    ) -> Source
    where
        S: AsRef<str>,
    {
        if let Some(decode_fn_name) = self.gen_decode_fn_name(type_tag) {
            Source::default()
                .with_literal(format!("let {}: {} = {{", var, self.typing.owned(type_tag)))
                .with_semi(format!(
                    "let decoder: {} = {}::default()",
                    self.naming.decoder_type_name, self.naming.decoder_type_name,
                ))
                .with_semi("let first: u8 = enc::read_single_byte(r)?")
                .with_literal(format!(
                    "decoder.{}({}, r, first)?",
                    decode_fn_name,
                    self.typing.wire_type_exp(type_tag, fixed)
                ))
                .with_literal("};")
        } else {
            unimplemented!("slice of slice requires recursion")
        }
    }

    fn gen_decode_fn_name(&self, type_tag: &TypeTag) -> Option<String> {
        let tag: String = match type_tag {
            TypeTag::Primitive(primitive) => primitive.as_ref().to_string(),
            TypeTag::Special(special) => special.as_ref().to_string(),
            TypeTag::Named(_) => "packet".to_string(),
            TypeTag::Slice(base) => match base.as_ref() {
                TypeTag::Primitive(primitive) => format!("{}_slice", primitive.as_ref()),
                TypeTag::Special(special) => format!("{}_slice", special.as_ref()),
                TypeTag::Named(_) => "packet_slice".to_string(),
                TypeTag::Slice(_) => return None,
            },
        };
        Some(format!("decode_{}", tag))
    }
}
