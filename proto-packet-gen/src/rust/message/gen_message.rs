use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::Message;

use crate::rust::{GenMessageEncode, GenMessageField, GenMessageStruct, Naming, Typing};
use crate::GenError;

/// Responsible for generating code for message types.
#[derive(Clone, Debug)]
pub struct GenMessage<'a> {
    naming: &'a Naming,
    typing: &'a Typing,
}

impl<'a> GenMessage<'a> {
    //! Construction

    /// Creates a new gen message.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self { naming, typing }
    }
}

impl<'a> GenMessage<'a> {
    //! Gen

    /// Generates the source code for the message.
    pub fn gen(&self, message: &Message) -> Result<Source, GenError> {
        let mut source: Source = Source::default();

        let gen: GenMessageStruct = GenMessageStruct::new(self.naming, self.typing);
        source.add_statement(gen.gen_struct(message)?);

        let gen: GenMessageField = GenMessageField::new(self.naming, self.typing);
        for field in message.fields() {
            source.add_statement(EmptyLine::default());
            source.add_statement(gen.gen_field(message, field)?);
        }

        let gen: GenMessageEncode = GenMessageEncode::new(self.naming, self.typing);
        source.add_statement(EmptyLine::default());
        source.add_statement(gen.gen_impl_encoded_len(message)?);
        source.add_statement(EmptyLine::default());
        source.add_statement(gen.gen_impl_encode_to_slice(message)?);
        source.add_statement(EmptyLine::default());
        source.add_statement(gen.gen_impl_encode_to_write(message)?);

        Ok(source)
    }
}
