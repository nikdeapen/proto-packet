use crate::rust::GenRust;
use code_gen::rust::Access::Public;
use code_gen::rust::{
    Function, ImplBlock, RustType, Signature, WithAccess, WithComments, WithFnGenerics,
    WithFunctions, WithResult, WithVarParams,
};
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet_tree::{Struct, WithFieldName, WithTypeTag};

impl GenRust {
    //! Gen Constructor

    /// Generates the constructor impl block for the struct `s`.
    pub fn gen_constructor_struct(&self, s: &Struct) -> Source {
        let mut new_sig: Signature = Signature::from("new").with_result("Self");
        for (i, field) in s.fields().iter().enumerate() {
            new_sig.add_generic((
                format!("F{}", i),
                RustType::from("Into")
                    .with_generic(self.typing.field_type(field.type_tag(), false)),
            ));
            new_sig.add_param((
                self.naming.field_name(field.field_name()),
                format!("F{}", i),
            ));
        }

        let mut new_fn: Function = Function::from(new_sig)
            .with_access(Public)
            .with_comment(format!(" Creates a new `{}`.", self.naming.type_name(s)));
        new_fn.add_literal("Self {");
        for field in s.fields() {
            new_fn.add_literal(format!(
                "    {}: {}.into(),",
                self.naming.field_name(field.field_name()),
                self.naming.field_name(field.field_name()),
            ));
        }
        new_fn.add_literal("}");

        let block: ImplBlock = ImplBlock::from(self.naming.type_name(s))
            .with_comment(" Construction")
            .with_function(new_fn);

        Source::default()
            .with_statement(EmptyLine::default())
            .with_statement(block)
    }
}
