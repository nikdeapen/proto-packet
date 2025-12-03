use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet_tree::{Message, ModPathRef};

impl GenRust {
    //! Gen Message

    /// Generates the source code for the message `m`.
    pub(in crate::rust) fn gen_message(&self, mod_path: ModPathRef, m: &Message) -> Source {
        Source::default()
            .with_statement(self.gen_message_type_dec(m))
            .with_statement(self.gen_message_impls(m))
            .with_statement(self.gen_message_fields(m))
            .with_statement(self.gen_message_encode(m))
            .with_statement(self.gen_message_decode(mod_path, m))
    }
}
