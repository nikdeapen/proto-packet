use crate::rust::GenRust;
use code_gen::rust::Access::Public;
use code_gen::rust::Receiver::Borrowed;
use code_gen::rust::{
    Function, RustType, Signature, WithAccess, WithComments, WithReceiver, WithResult,
};
use code_gen::WithStatements;
use proto_packet_tree::{WithFieldName, WithTypeTag};

impl GenRust {
    //! Gen Fields: Getter

    /// Generates the getter function for the `field`.
    pub(in crate::rust) fn gen_field_getter<F>(&self, field: &F, optional: bool) -> Function
    where
        F: WithFieldName + WithTypeTag,
    {
        let fn_name: String = self.naming.field_name(field);
        let result_type: RustType = self.typing.borrowed_field_type(field.type_tag(), optional);
        let signature: Signature = Signature::from(fn_name)
            .with_receiver(Borrowed)
            .with_result(result_type);
        Function::from(signature)
            .with_comment(format!(" Gets the field: `{}`.", field.field_name()))
            .with_access(Public)
            .with_literal(
                self.typing
                    .borrow_method(field.type_tag())
                    .gen_field_exp(self.naming.field_name(field), optional),
            )
    }
}
