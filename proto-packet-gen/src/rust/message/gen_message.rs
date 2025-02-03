use code_gen::{Source, WithStatements};

use proto_packet_tree::Message;

use crate::rust::GenRust;

impl GenRust {
    //! Gen Message

    /// Generates the code for the `message`.
    pub fn gen_message(&self, message: &Message) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(self.gen_message_imports());
        source.add_statement(self.gen_message_struct(message));
        source.add_statement(self.gen_message_impls(message));
        source.add_statement(self.gen_message_fields(message));
        source.add_statement(self.gen_message_encode(message));
        source.add_statement(self.gen_message_decode(message));

        source
    }
}
