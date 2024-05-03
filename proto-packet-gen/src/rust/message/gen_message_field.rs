use code_gen::rust::{
    gen_builder, gen_getter_copy, gen_getter_field_exp, gen_setter_copy, gen_setter_mem_replace,
    Function, ImplBlock, TypeTag as RustType, WithComments, WithFunctions,
};

use proto_packet_tree::{Message, MessageField, WithName, WithTypeTag};

use crate::rust::{Naming, Typing};
use crate::GenError;

/// Responsible for generating struct impl blocks for message fields.
#[derive(Copy, Clone, Debug)]
pub struct GenMessageField<'a> {
    naming: &'a Naming,
    typing: &'a Typing,
}

impl<'a> GenMessageField<'a> {
    //! Construction

    /// Creates a new generator message field.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self { naming, typing }
    }
}

impl<'a> GenMessageField<'a> {
    //! Gen

    /// Generates the impl block for the message field.
    pub fn gen_field(
        &self,
        message: &Message,
        field: &MessageField,
    ) -> Result<ImplBlock, GenError> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();

        if let Some(field_number) = field.field_number() {
            block.add_comment(format!(
                "Field: {} {} = {};",
                field.name(),
                field.type_tag(),
                field_number
            ));
        } else {
            block.add_comment(format!("Field: {} {};", field.name(), field.type_tag()));
        }

        self.gen_getters(&mut block, field)?;
        self.gen_setters(&mut block, field)?;
        self.gen_builders(&mut block, field)?;

        Ok(block)
    }
}

impl<'a> GenMessageField<'a> {
    //! Get

    /// Generates the getter functions for the field.
    fn gen_getters(&self, b: &mut ImplBlock, field: &MessageField) -> Result<(), GenError> {
        let name: String = self.naming.field_name(field.name())?;
        let tag: RustType = self.typing.borrowed_type(field.type_tag())?.to_option();
        let comment: String = format!("Gets the field: `{}`.", field.name());
        let function: Function = if self.typing.is_copy(field.type_tag())? {
            gen_getter_copy(name, tag)
        } else {
            gen_getter_field_exp(name, tag, |field_name| {
                format!("self.{}.as_deref()", field_name)
            })
        };
        b.add_function(function.with_comment(comment));
        Ok(())
    }
}

impl<'a> GenMessageField<'a> {
    //! Set

    /// Generates the setter functions for the field.
    fn gen_setters(&self, b: &mut ImplBlock, field: &MessageField) -> Result<(), GenError> {
        let field_name: String = self.naming.field_name(field.name())?;
        let rust_type: RustType = self.typing.field_type(field.type_tag())?.to_option();
        let comment: String = format!(
            "Sets the field: `{}`. Returns the previous value.",
            field.name()
        );
        let function: Function = if self.typing.is_copy(field.type_tag())? {
            gen_setter_copy(field_name, rust_type)
        } else {
            gen_setter_mem_replace(field_name, rust_type)
        };
        b.add_function(function.with_comment(comment));
        Ok(())
    }
}

impl<'a> GenMessageField<'a> {
    //! Build

    /// Generates the builder functions for the field.
    fn gen_builders(&self, b: &mut ImplBlock, field: &MessageField) -> Result<(), GenError> {
        let name: String = self.naming.field_name(field.name())?;
        let tag: RustType = self.typing.field_type(field.type_tag())?.to_option();
        let function: Function = gen_builder(name, tag).with_comment(format!(
            "Builds the field: `{}`. Returns the struct itself.",
            field.name()
        ));
        b.add_function(function);
        Ok(())
    }
}
