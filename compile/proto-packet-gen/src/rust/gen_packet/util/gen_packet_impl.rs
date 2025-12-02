use crate::rust::GenRust;
use code_gen::rust::{Function, ImplBlock, Signature, WithFunctions, WithResult};
use code_gen::WithStatements;
use proto_packet::io::WireType;
use proto_packet_tree::WithTypeName;

impl GenRust {
    //! Util: Gen Packet Impls

    /// Generates the `Packet` impl block for the `element`.
    pub(in crate::rust::gen_packet) fn gen_packet_impl<T>(
        &self,
        element: &T,
        wire_type: WireType,
    ) -> ImplBlock
    where
        T: WithTypeName,
    {
        let wire_type_fn: Function =
            Function::from(Signature::from("wire_type").with_result("proto_packet::io::WireType"))
                .with_literal(format!("proto_packet::io::WireType::{}", wire_type));
        ImplBlock::from(self.naming.type_name(element))
            .with_for_trait("proto_packet::Packet")
            .with_function(wire_type_fn)
    }
}
