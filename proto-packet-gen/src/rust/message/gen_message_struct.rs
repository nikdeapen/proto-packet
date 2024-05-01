use code_gen::rust::Access::Public;
use code_gen::rust::{
    Struct, StructField, TypeTag as RustType, WithAccess, WithComments, WithStructFields,
};

use proto_packet_tree::{Message, MessageField, WithName, WithTypeTag};

use crate::rust::{Naming, Typing};
use crate::GenError;

/// Responsible for generating struct declarations for message types.
#[derive(Copy, Clone, Debug)]
pub struct GenMessageStruct<'a> {
    naming: &'a Naming,
    typing: &'a Typing,
}

impl<'a> GenMessageStruct<'a> {
    //! Construction

    /// Creates a new generator message struct.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self { naming, typing }
    }
}

impl<'a> GenMessageStruct<'a> {
    //! Gen

    /// Generates the struct declaration for the message.
    pub fn gen_struct(&self, message: &Message) -> Result<Struct, GenError> {
        let mut s: Struct = self.naming.type_name(message.name())?.into();
        s.set_access(Public);

        self.gen_comments(message, &mut s)?;

        for field in message.fields() {
            self.gen_field(&mut s, field)?;
        }

        Ok(s)
    }

    /// Generates the comments for the message struct.
    pub fn gen_comments(&self, message: &Message, s: &mut Struct) -> Result<(), GenError> {
        let comments: String = message.to_string();
        comments
            .lines()
            .for_each(|line| s.add_comment(format!("{}", line)));
        Ok(())
    }

    /// Generates the code for the field.
    fn gen_field(&self, s: &mut Struct, field: &MessageField) -> Result<(), GenError> {
        let name: String = self.naming.field_name(field.name())?;
        let tag: RustType = self.typing.field_type(field.type_tag())?.to_option();
        let field: StructField = (name, tag).into();
        s.add_field(field);
        Ok(())
    }
}
