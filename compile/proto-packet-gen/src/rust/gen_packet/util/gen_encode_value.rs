use crate::rust::util::EncodeOp;
use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet_tree::TypeTag;

impl GenRust {
    //! Gen Encode Value

    /// Generates the source to encode a value.
    ///
    /// # Expected in Scope
    ///   `value_exp: &T`             (where `T` is the type of value)
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
            .with_semi(format!(
                "let encoder: proto_packet::io::Encoder<{}> = proto_packet::io::Encoder::new({}, {})",
                self.typing.owned(type_tag),
                value_exp,
                fixed
            ))
            .with_semi(format!("encoded_len += encoder.{}?", op.encode_call()))
    }
}
