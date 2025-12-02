use crate::rust::GenRust;
use code_gen::rust::Access::Public;
use code_gen::rust::Receiver::OwnedMut;
use code_gen::rust::{
    Function, RustType, Signature, WithAccess, WithComments, WithFnGenerics, WithReceiver,
    WithResult, WithVarParams,
};
use code_gen::WithStatements;
use proto_packet_tree::{WithFieldName, WithTypeTag};

impl GenRust {
    //! Gen Field: Builder

    /// Generates the builder function for the `field`.
    pub(in crate::rust) fn gen_field_builder<F>(&self, field: &F, optional: bool) -> Function
    where
        F: WithFieldName + WithTypeTag,
    {
        let fn_name: String = format!("with_{}", self.naming.field_name(field));
        let field_type: RustType = self.typing.field_type(field.type_tag(), optional);
        let signature: Signature = Signature::from(fn_name)
            .with_generic(("T", RustType::from("Into").with_generic(field_type.clone())))
            .with_receiver(OwnedMut)
            .with_param((self.naming.field_name(field), "T"))
            .with_result("Self");
        Function::from(signature)
            .with_comment(format!(
                " Sets the field: `{}`. Returns the struct itself.",
                field.field_name()
            ))
            .with_access(Public)
            .with_semi(format!(
                "self.set_{}({})",
                self.naming.field_name(field),
                self.naming.field_name(field)
            ))
            .with_literal("self")
    }
}
