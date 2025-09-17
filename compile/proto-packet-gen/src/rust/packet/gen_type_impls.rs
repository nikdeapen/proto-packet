use crate::rust::GenRust;
use code_gen::rust::Receiver::Borrowed;
use code_gen::rust::{
    Function, ImplBlock, MatchCase, MatchStatement, RustPrimitive, Signature, WithFunctions,
    WithReceiver, WithResult,
};
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet::io::WireType;
use proto_packet::io::WireType::{LengthPrefixed, VarInt};
use proto_packet::PacketType;
use proto_packet_tree::{
    Enum, Message, Struct, Variant, WithCaseName, WithTagNumber, WithTypeName,
};

impl GenRust {
    //! Gen Type Impls

    pub(in crate::rust) fn gen_type_impls_struct(&self, s: &Struct) -> Source {
        self.gen_type_impls(s, PacketType::Struct, LengthPrefixed, vec![])
    }

    pub(in crate::rust) fn gen_type_impls_message(&self, m: &Message) -> Source {
        self.gen_type_impls(m, PacketType::Message, LengthPrefixed, vec![])
    }

    pub(in crate::rust) fn gen_type_impls_enum(&self, e: &Enum) -> Source {
        Source::default()
            .with_statement(self.gen_type_impls(e, PacketType::Enum, VarInt, vec![]))
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_impl_with_tag_number(e, self.gen_enum_tag_number_fn_source(e)))
    }

    pub(in crate::rust) fn gen_type_impls_variant(&self, v: &Variant) -> Source {
        Source::default()
            .with_statement(self.gen_type_impls(v, PacketType::Variant, LengthPrefixed, vec![]))
            .with_statement(EmptyLine::default())
            .with_statement(
                self.gen_impl_with_tag_number(v, self.gen_variant_tag_number_fn_source(v)),
            )
    }
}

impl GenRust {
    //! Gen Type Impls: Generic

    fn gen_type_impls<T>(
        &self,
        element: &T,
        packet_type: PacketType,
        wire_type: WireType,
        mut type_fns: Vec<Function>,
    ) -> Source
    where
        T: WithTypeName,
    {
        let mut type_impl: ImplBlock =
            ImplBlock::from(self.naming.type_name(element)).with_for_trait(packet_type.to_string());
        type_fns.drain(..).for_each(|f| type_impl.add_function(f));

        Source::default()
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_packet_impl(element, wire_type, packet_type))
            .with_statement(EmptyLine::default())
            .with_statement(type_impl)
    }

    fn gen_packet_impl<T>(
        &self,
        element: &T,
        wire_type: WireType,
        packet_type: PacketType,
    ) -> ImplBlock
    where
        T: WithTypeName,
    {
        let wire_type: Function =
            Function::from(Signature::from("wire_type").with_result("WireType"))
                .with_literal(format!("WireType::{}", wire_type));
        let packet_type: Function =
            Function::from(Signature::from("packet_type").with_result("PacketType"))
                .with_literal(format!("PacketType::{}", packet_type));
        ImplBlock::from(self.naming.type_name(element))
            .with_for_trait("Packet")
            .with_function(wire_type)
            .with_function(packet_type)
    }
}

impl GenRust {
    //! Gen Type Impls: Tag Number

    fn gen_impl_with_tag_number<E>(&self, element: &E, fn_source: MatchStatement) -> ImplBlock
    where
        E: WithTypeName,
    {
        ImplBlock::from(self.naming.type_name(element))
            .with_for_trait("WithTagNumber")
            .with_function(
                Function::from(
                    Signature::from("tag_number")
                        .with_receiver(Borrowed)
                        .with_result(self.naming.tag_number_type_name.as_str()),
                )
                .with_statement(fn_source)
                .with_literal(format!(
                    "unsafe {{ {}::new_unchecked({}) }}",
                    self.naming.tag_number_type_name, "tag_number"
                )),
            )
    }

    fn gen_enum_tag_number_fn_source(&self, e: &Enum) -> MatchStatement {
        let mut match_statement: MatchStatement = MatchStatement::from("self")
            .with_assignment(("tag_number", RustPrimitive::UnsignedInt32));
        for case in e.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!("Self::{}", self.naming.case_name(case.case_name())))
                    .with_literal(case.tag_number().to_string()),
            )
        }
        match_statement
    }

    fn gen_variant_tag_number_fn_source(&self, e: &Variant) -> MatchStatement {
        let mut match_statement: MatchStatement = MatchStatement::from("self")
            .with_assignment(("tag_number", RustPrimitive::UnsignedInt32));
        for case in e.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!(
                    "Self::{}(_)",
                    self.naming.case_name(case.case_name())
                ))
                .with_literal(case.tag_number().to_string()),
            )
        }
        match_statement
    }
}
