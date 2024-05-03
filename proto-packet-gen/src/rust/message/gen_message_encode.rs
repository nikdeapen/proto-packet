use code_gen::rust::{
    Function, ImplBlock, PrimitiveType as RustPrimitive, Receiver, Reference, Signature,
    TypeTag as RustType, WithFnGenerics, WithFunctions, WithReceiver, WithResult, WithUnsafeFlag,
    WithVarParams,
};
use code_gen::{Literal, Semi, WithStatements};

use proto_packet_tree::{
    Message, MessageField, PrimitiveType, SpecialType, TypeTag, WithName, WithTypeTag,
};

use crate::rust::{Naming, Typing};
use crate::GenError;

/// Responsible for generating struct impl blocks for message fields.
#[derive(Copy, Clone, Debug)]
pub struct GenMessageEncode<'a> {
    naming: &'a Naming,
    _typing: &'a Typing,
}

impl<'a> GenMessageEncode<'a> {
    //! Construction

    /// Creates a new generator message field.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self {
            naming,
            _typing: typing,
        }
    }
}

impl<'a> GenMessageEncode<'a> {
    //! Field Expression

    /// Generates the field expression for the field.
    fn gen_field_exp(
        &self,
        declared_name: &str,
        type_tag: &TypeTag,
        field_number: u32,
    ) -> Result<String, GenError> {
        match type_tag {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => {
                    self.field_exp_int(declared_name, false, 8, None, field_number)
                }
                PrimitiveType::UnsignedInt16 => {
                    // todo -- supported fixed fields
                    self.field_exp_int(declared_name, false, 16, Some(false), field_number)
                }
                PrimitiveType::UnsignedInt32 => {
                    // todo -- supported fixed fields
                    self.field_exp_int(declared_name, false, 32, Some(false), field_number)
                }
                PrimitiveType::UnsignedInt64 => {
                    // todo -- supported fixed fields
                    self.field_exp_int(declared_name, false, 64, Some(false), field_number)
                }
                PrimitiveType::UnsignedInt128 => {
                    // todo -- supported fixed fields
                    self.field_exp_int(declared_name, false, 128, Some(false), field_number)
                }
            },
            TypeTag::Special(special) => match special {
                SpecialType::String => self.field_exp_string(declared_name, field_number),
                SpecialType::UniqueIdentifier => self.field_exp_uuid(declared_name, field_number),
            },
        }
    }

    /// Gets the field constructor expression string.
    fn field_exp_int(
        &self,
        declared_name: &str,
        signed: bool,
        bits: u32,
        fixed: Option<bool>,
        field_number: u32,
    ) -> Result<String, GenError> {
        let signed: &str = if signed { "Signed" } else { "Unsigned" };
        let name: String = self.naming.field_name(declared_name)?;
        let result: String = if let Some(fixed) = fixed {
            format!(
                "{}Int{}Field::new({}, {}, self.{})",
                signed, bits, field_number, fixed, name
            )
        } else {
            format!(
                "{}Int{}Field::new({}, self.{})",
                signed, bits, field_number, name
            )
        };
        Ok(result)
    }

    fn field_exp_string(&self, declared_name: &str, field_number: u32) -> Result<String, GenError> {
        let field_name: String = self.naming.field_name(declared_name)?;
        let result: String = format!(
            "BytesField::new({}, self.{}.as_deref())",
            field_number, field_name
        );
        Ok(result)
    }

    fn field_exp_uuid(&self, declared_name: &str, field_number: u32) -> Result<String, GenError> {
        let field_name: String = self.naming.field_name(declared_name)?;
        let result: String = format!(
            "UniqueIdentifierField::new({}, self.{})",
            field_number, field_name
        );
        Ok(result)
    }
}

impl<'a> GenMessageEncode<'a> {
    //! EncodedLength

    /// Generates the impl block for implementing `EncodedLen`.
    pub fn gen_impl_encoded_len(&self, message: &Message) -> Result<ImplBlock, GenError> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();
        block.set_for_trait("EncodedLen");

        let signature: Signature = Signature::from("encoded_len")
            .with_receiver(Receiver::Borrowed)
            .with_result(RustPrimitive::UnsignedIntSize);
        let mut function: Function = Function::from(signature);
        function.add_statement(Semi::from("let mut encoded_len: usize = 0"));

        for field in message.fields() {
            function.add_statement(self.gen_encoded_len_statement(field)?);
        }

        function.add_expression_statement(Literal::from("encoded_len"));
        block.add_function(function);

        Ok(block)
    }

    /// Generates the encoded length statement for the field.
    /// todo -- optimize literal allocation
    fn gen_encoded_len_statement(&self, field: &MessageField) -> Result<Semi<Literal>, GenError> {
        if let Some(field_number) = field.field_number() {
            let field_exp: String =
                self.gen_field_exp(field.name(), field.type_tag(), field_number)?;
            // todo -- remove literal
            Ok(Semi::from(Literal::from(format!(
                "encoded_len += {}.encoded_len()",
                field_exp
            ))))
        } else {
            unimplemented!("required fields not yet supported")
        }
    }
}

impl<'a> GenMessageEncode<'a> {
    //! EncodeToSlice

    /// Generates the impl block for implementing `EncodedLen`.
    pub fn gen_impl_encode_to_slice(&self, message: &Message) -> Result<ImplBlock, GenError> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();
        block.set_for_trait("EncodeToSlice");

        let signature: Signature = Signature::from("encode_to_slice_unchecked")
            .with_unsafe()
            .with_receiver(Receiver::Borrowed)
            .with_param((
                "target",
                RustPrimitive::UnsignedInt8
                    .to_type_tag()
                    .to_slice()
                    .to_ref_type(Reference::default().with_mut()),
            ))
            .with_result(RustPrimitive::UnsignedIntSize);
        let mut function: Function = Function::from(signature);

        function.add_statement(Semi::from("let mut encoded_len: usize = 0"));
        for field in message.fields() {
            function.add_statement(self.gen_encode_to_slice_statement(field)?);
        }
        function.add_expression_statement(Literal::from("encoded_len"));

        block.add_function(function);

        Ok(block)
    }

    fn gen_encode_to_slice_statement(
        &self,
        field: &MessageField,
    ) -> Result<Semi<Literal>, GenError> {
        if let Some(field_number) = field.field_number() {
            let field_exp: String =
                self.gen_field_exp(field.name(), field.type_tag(), field_number)?;
            // todo -- remove literal
            Ok(Semi::from(Literal::from(format!(
                "encoded_len += {}.encode_to_slice_unchecked(target)",
                field_exp
            ))))
        } else {
            unimplemented!("required fields not yet supported")
        }
    }
}

impl<'a> GenMessageEncode<'a> {
    //! EncodeToWrite

    /// Generates the impl block for implementing `EncodeToWrite`.
    pub fn gen_impl_encode_to_write(&self, message: &Message) -> Result<ImplBlock, GenError> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();
        block.set_for_trait("EncodeToWrite");

        let result_type: RustType = RustType::Named("Result".to_string())
            .with_generic(RustPrimitive::UnsignedIntSize)
            .with_generic("io::Error");
        let signature: Signature = Signature::from("encode_to_write")
            .with_receiver(Receiver::Borrowed)
            .with_generic(("W", "io::Write"))
            .with_param((
                "w",
                RustType::Named("W".to_string()).to_ref_type(Reference::default().with_mut()),
            ))
            .with_result(result_type);
        let mut function: Function = Function::from(signature);

        function.add_statement(Semi::from("let mut encoded_len: usize = 0"));
        for field in message.fields() {
            function.add_statement(self.gen_encode_to_write_statement(field)?);
        }
        function.add_expression_statement(Literal::from("Ok(encoded_len)"));

        block.add_function(function);

        Ok(block)
    }

    fn gen_encode_to_write_statement(
        &self,
        field: &MessageField,
    ) -> Result<Semi<Literal>, GenError> {
        if let Some(field_number) = field.field_number() {
            let field_exp: String =
                self.gen_field_exp(field.name(), field.type_tag(), field_number)?;
            // todo -- remove literal
            Ok(Semi::from(Literal::from(format!(
                "encoded_len += {}.encode_to_write(w)?",
                field_exp
            ))))
        } else {
            unimplemented!("required fields not yet supported")
        }
    }
}
