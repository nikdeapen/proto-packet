use crate::rust::GenRust;
use code_gen::rust::{ImplBlock, WithComments as RustWithComments, WithFunctions};
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet::io::TagNumber;
use proto_packet_tree::{
    Message, MessageField, Struct, StructField, WithComments, WithFieldName, WithTagNumber,
    WithTypeName, WithTypeTag,
};

impl GenRust {
    //! Gen Fields

    /// Generates the field impl blocks for the struct `s`.
    pub(in crate::rust) fn gen_fields_struct(&self, s: &Struct) -> Source {
        let mut source: Source = Source::default();

        for field in s.fields() {
            source.add_statement(EmptyLine::default());
            source.add_statement(self.gen_field_impl_block_struct(s, field));
        }

        source
    }

    /// Generates the field impl blocks for the message `m`.
    pub(in crate::rust) fn gen_fields_message(&self, m: &Message) -> Source {
        let mut source: Source = Source::default();

        for field in m.fields() {
            source.add_statement(EmptyLine::default());
            source.add_statement(self.gen_field_impl_block_message(m, field));
        }

        source
    }
}

impl GenRust {
    //! Gen Fields: Impl Block

    /// Generates the `field` impl block for the struct `s`.
    fn gen_field_impl_block_struct(&self, s: &Struct, field: &StructField) -> ImplBlock {
        self.gen_field_impl_block(s, field, None)
            .with_function(self.gen_field_getter(field, false))
            .with_function(self.gen_field_setter(field, false))
            .with_function(self.gen_builder(field, false))
    }

    /// Generates the `field` impl block for the message `m`.
    fn gen_field_impl_block_message(&self, m: &Message, field: &MessageField) -> ImplBlock {
        self.gen_field_impl_block(m, field, Some(field.tag_number()))
            .with_function(self.gen_field_getter(field, true))
            .with_function(self.gen_field_setter(field, true))
            .with_function(self.gen_builder(field, true))
    }

    /// Generates the generic field impl block for the `element` and `field`.
    fn gen_field_impl_block<T, F>(
        &self,
        element: &T,
        field: &F,
        tag_number: Option<TagNumber>,
    ) -> ImplBlock
    where
        T: WithTypeName,
        F: WithComments + WithFieldName + WithTypeTag,
    {
        let mut block: ImplBlock = ImplBlock::from(self.naming.type_name(element.type_name()));

        block.add_comment(format!(" Field: `{}`", field.field_name()));
        block.add_comment(" ");
        for comment in field.comments() {
            block.add_comment(format!(" //{}", comment));
        }
        if let Some(tag_number) = tag_number {
            block.add_comment(format!(
                " {}: {} = {};",
                field.field_name(),
                field.type_tag(),
                tag_number
            ))
        } else {
            block.add_comment(format!(" {}: {};", field.field_name(), field.type_tag()))
        }

        block
    }
}
