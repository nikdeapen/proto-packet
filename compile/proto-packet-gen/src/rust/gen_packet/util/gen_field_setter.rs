use crate::rust::GenRust;
use code_gen::rust::Access::Public;
use code_gen::rust::Receiver::BorrowedMut;
use code_gen::rust::{
    Function, RustType, Signature, WithAccess, WithComments, WithFnGenerics, WithReceiver,
    WithResult, WithVarParams,
};
use code_gen::WithStatements;
use proto_packet_tree::{WithFieldName, WithTypeTag};

impl GenRust {
    //! Gen Fields: Setter

    /// Generates the setter function for the `field`.
    pub(in crate::rust) fn gen_field_setter<F>(&self, field: &F, optional: bool) -> Function
    where
        F: WithFieldName + WithTypeTag,
    {
        let fn_name: String = format!("set_{}", self.naming.field_name(field));
        let field_type: RustType = self.typing.field_type(field.type_tag(), optional);
        let signature: Signature = Signature::from(fn_name)
            .with_generic(("T", RustType::from("Into").with_generic(field_type.clone())))
            .with_receiver(BorrowedMut)
            .with_param((self.naming.field_name(field), "T"))
            .with_result(field_type.clone());
        let mut function: Function = Function::from(signature)
            .with_comment(format!(
                " Sets the field: `{}`. Returns the previous value.",
                self.naming.field_name(field)
            ))
            .with_access(Public);

        function.add_statement(
            self.typing
                .borrow_method(field.type_tag())
                .set_source(self.naming.field_name(field).as_str(), &field_type),
        );

        function
    }
}
