use crate::rust::GenRust;
use code_gen::rust::{ImplBlock, WithFunctions};
use code_gen::{Source, WithStatements};
use proto_packet_tree::{Message, MessageField, WithTagNumber};

impl GenRust {
    //! Gen Message: Fields

    /// Generates the field impl blocks for the message `m`.
    pub(in crate::rust) fn gen_message_fields(&self, m: &Message) -> Source {
        let mut source: Source = Source::default();

        for field in m.fields() {
            source.add_empty_line();
            source.add_statement(self.gen_message_fields_impl_block(m, field));
        }

        source
    }

    /// Generates the `field` impl block for the message `m`.
    fn gen_message_fields_impl_block(&self, m: &Message, field: &MessageField) -> ImplBlock {
        self.gen_field_impl_block(m, field, Some(field.tag_number()))
            .with_function(self.gen_field_getter(field, true))
            .with_function(self.gen_field_setter(field, true))
            .with_function(self.gen_field_builder(field, true))
    }
}
