use code_gen::rust::Access::Public;
use code_gen::rust::{
    FieldExp, Function, ImplBlock, Receiver, RustType, Signature, VarInit, WithAccess,
    WithComments as RustWithComments, WithFnGenerics, WithFunctions, WithReceiver, WithResult,
    WithVarParams,
};
use code_gen::{EmptyLine, Literal, Source, WithStatements};

use proto_packet_tree::{
    Message, MessageField, WithComments, WithFieldName, WithTagNumberOptional, WithTypeName,
    WithTypeTag,
};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Message Fields

    pub(in crate::rust::message) fn gen_message_fields(&self, message: &Message) -> Source {
        let mut source: Source = Source::default();
        for field in message.fields() {
            source.add_statement(EmptyLine::default());
            source.add_statement(self.gen_field(message, field));
        }
        source
    }

    fn gen_field(&self, message: &Message, field: &MessageField) -> ImplBlock {
        let mut b: ImplBlock = ImplBlock::from(self.naming.type_name(message.type_name()));

        self.gen_field_comments(&mut b, field);
        b.add_function(self.gen_field_getter(field));
        b.add_function(self.gen_field_setter(field));
        b.add_function(self.gen_field_builder(field));

        b
    }

    fn gen_field_comments(&self, b: &mut ImplBlock, field: &MessageField) {
        b.add_comment(format!(" Field `{}`", field.field_name()));
        b.add_comment("");
        for comment in field.comments() {
            b.add_comment(format!(" //{}", comment));
        }
        if let Some(tag_number) = field.tag_number() {
            b.add_comment(format!(
                " {}: {} = {};",
                field.field_name(),
                field.type_tag(),
                tag_number
            ));
        } else {
            b.add_comment(format!(" {} {};", field.field_name(), field.type_tag()));
        }
    }

    /// Generates the getter function for the `field`.
    fn gen_field_getter(&self, field: &MessageField) -> Function {
        let field_name: String = self.naming.field_name(field.field_name());
        let result_type: RustType = self.typing.borrowed_type(field.type_tag());

        let signature: Signature = Signature::from(field_name.clone())
            .with_receiver(Receiver::Borrowed)
            .with_result(result_type.to_option());

        let mut function: Function = signature.into();
        function.set_access(Public);

        function.add_comment(format!(" Gets the field: `{}`.", field_name));

        if self.typing.is_copy(field.type_tag()) {
            function.add_expression_statement(FieldExp::from(field_name));
        } else if self.typing.is_deref(field.type_tag()) {
            function.add_literal(format!("self.{}.as_deref()", field_name))
        } else {
            function.add_literal(format!("self.{}.as_ref()", field_name))
        }

        function
    }

    /// Generates the setter function for the `field`.
    fn gen_field_setter(&self, field: &MessageField) -> Function {
        let field_name: String = self.naming.field_name(field.field_name());
        let fn_name: String = format!("set_{}", field_name);
        let field_type: RustType = self.typing.field_type(field.type_tag());

        let signature: Signature = Signature::from(fn_name)
            .with_generic((
                "O",
                RustType::from("Into").with_generic(field_type.clone().to_option()),
            ))
            .with_receiver(Receiver::BorrowedMut)
            .with_param((field_name.clone(), RustType::from("O")))
            .with_result(field_type.clone().to_option());

        let mut function: Function = signature.into();
        function.set_access(Public);

        function.add_comment(format!(
            " Sets the field: `{}`. Returns the previous value.",
            field_name
        ));

        if self.typing.is_copy(field.type_tag()) {
            let old_name: String = format!("old_{}", field_name);
            function.add_statement(VarInit::from((
                (old_name.clone(), field_type.to_option()),
                FieldExp::from(field_name.clone()),
            )));
            function.add_semi(format!("self.{} = {}.into()", field_name, field_name));
            function.add_literal(old_name);
        } else {
            function.add_statement(VarInit::from((
                (field_name.clone(), field_type.to_option()),
                Literal::from(format!("{}.into()", field_name)),
            )));
            function.add_literal(format!(
                "std::mem::replace(&mut self.{}, {})",
                field_name, field_name
            ))
        }

        function
    }

    /// Generates the builder function for the `field`.
    fn gen_field_builder(&self, field: &MessageField) -> Function {
        let field_name: String = self.naming.field_name(field.field_name());
        let fn_name: String = format!("with_{}", field_name);
        let field_type: RustType = self.typing.field_type(field.type_tag());

        let signature: Signature = Signature::from(fn_name)
            .with_generic((
                "O",
                RustType::from("Into").with_generic(field_type.clone().to_option()),
            ))
            .with_receiver(Receiver::OwnedMut)
            .with_param((field_name.clone(), RustType::from("O")))
            .with_result(RustType::SelfType);

        let mut function: Function = signature.into();
        function.set_access(Public);

        function.add_comment(format!(
            " Builds the field: `{}`. Returns the struct itself.",
            field_name
        ));

        function.add_semi(format!("self.{} = {}.into()", field_name, field_name));
        function.add_literal("self");

        function
    }
}
