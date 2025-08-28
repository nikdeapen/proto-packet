use crate::rust::{EncodeOp, GenRust};
use code_gen::{Source, WithStatements};
use proto_packet_tree::TypeTag;

impl GenRust {
    //! Gen Encode Value

    /// Generates the source to encode a value.
    ///
    /// # Expects in Scope
    ///   `value_exp: &T`
    ///   `encoded_len: usize`
    ///   `target: &mut [u8]`         (if `op == EncodeToSlice`)
    ///   `w: &mut std::io::Write`    (if `op == EncodeToWrite`)
    pub(in crate::rust) fn gen_encode_value(
        &self,
        value_exp: &str,
        type_tag: &TypeTag,
        fixed: bool,
        op: EncodeOp,
    ) -> Source {
        Source::default()
            .with_literal("encoded_len += {")
            .with_semi(format!(
                "let encoder: {}<{}> = {}::new({}, {})",
                self.naming.encoder_type_name,
                self.typing.owned(type_tag),
                self.naming.encoder_type_name,
                value_exp,
                fixed,
            ))
            .with_literal(format!("encoder.{}?", op.encode_call()))
            .with_semi("}")
    }
}
