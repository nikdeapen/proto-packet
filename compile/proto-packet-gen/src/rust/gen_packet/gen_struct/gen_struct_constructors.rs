use crate::rust::GenRust;
use code_gen::rust::Access;
use code_gen::rust::{
    Function, ImplBlock, RustType, Signature, WithAccess, WithComments, WithFnGenerics,
    WithFunctions, WithResult, WithVarParams,
};
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet_tree::{Struct, WithTypeTag};

impl GenRust {
    //! Gen Struct: Constructors

    /// Generates the constructors for the struct `s`.
    pub(in crate::rust::gen_packet::gen_struct) fn gen_struct_constructors(
        &self,
        s: &Struct,
    ) -> Source {
        Source::default()
            .with_statement(EmptyLine::default()) // todo -- with_empty_line()
            .with_statement(
                ImplBlock::from(self.naming.type_name(s))
                    .with_comment(" Construction")
                    .with_function(self.gen_struct_constructor(s, false))
                    .with_function(self.gen_struct_constructor(s, true)),
            )
    }

    fn gen_struct_constructor(&self, s: &Struct, generic: bool) -> Function {
        let signature: Signature = self.gen_struct_constructor_signature(s, generic);
        let mut function: Function = Function::from(signature)
            .with_access(Access::Public)
            .with_comment(format!(" Creates a new `{}`.", self.naming.type_name(s)));
        if !generic {
            function.set_const(true);
        }

        function.add_literal("Self {");
        for field in s.fields() {
            let field: String = self.naming.field_name(field);
            let field: String = if generic {
                format!("    {}: {}.into(),", field, field)
            } else {
                format!("    {},", field)
            };
            function.add_literal(field);
        }
        function.add_literal("}");

        function
    }

    fn gen_struct_constructor_signature(&self, s: &Struct, generic: bool) -> Signature {
        let name: &str = if generic { "from" } else { "new" };
        let mut signature: Signature = Signature::from(name).with_result("Self");

        for (i, field) in s.fields().iter().enumerate() {
            let field_name: String = self.naming.field_name(field);
            let field_type: RustType = self.typing.field_type(field.type_tag(), false);
            let generic_type: String = format!("F{}", i);
            if generic {
                signature.add_generic((
                    generic_type.clone(),
                    RustType::from("Into").with_generic(field_type.clone()),
                ));
            }
            signature.add_param((
                field_name,
                if generic {
                    RustType::from(generic_type)
                } else {
                    field_type
                },
            ));
        }

        signature
    }
}
