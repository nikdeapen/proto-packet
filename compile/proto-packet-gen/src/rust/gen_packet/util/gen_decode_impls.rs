use code_gen::rust::{
    Function, ImplBlock, Reference, RustPrimitive, RustType, Signature, WithFnGenerics,
    WithFunctions, WithResult, WithVarParams,
};
use code_gen::{Source, WithStatements};

use proto_packet_tree::TypeNameRef;

use crate::rust::GenRust;

impl GenRust {
    //! impl `enc::DecodeFromRead`

    /// Generates the impl block for the `enc::DecodeFromRead` trait.
    pub(in crate::rust) fn gen_decode_from_read_impl(
        &self,
        type_name: TypeNameRef,
        fn_source: Source,
    ) -> ImplBlock {
        let signature: Signature = Signature::from("decode_from_read")
            .with_generic(("R", "std::io::Read"))
            .with_param((
                "r",
                RustType::from("R").to_ref(Reference::default().with_mut()),
            ))
            .with_result(
                RustType::from("Result")
                    .with_generic(RustType::from("Self"))
                    .with_generic("enc::Error"),
            );
        ImplBlock::from(type_name)
            .with_for_trait("enc::DecodeFromRead")
            .with_function(Function::from(signature).with_statement(fn_source))
    }
}

impl GenRust {
    //! impl `enc::DecodeFromReadPrefix`

    /// Generates the impl block for the `enc::DecodeFromReadPrefix` trait.
    pub(in crate::rust) fn _gen_decode_from_read_prefix_impl(
        &self,
        type_name: TypeNameRef,
        fn_source: Source,
    ) -> ImplBlock {
        let signature: Signature = Signature::from("decode_from_read_prefix_with_first_byte")
            .with_generic(("R", "std::io::Read"))
            .with_param((
                "r",
                RustType::from("R").to_ref(Reference::default().with_mut()),
            ))
            .with_param(("first", RustPrimitive::UnsignedInt8))
            .with_result(
                RustType::from("Result")
                    .with_generic(RustType::from("Self"))
                    .with_generic("enc::Error"),
            );
        ImplBlock::from(type_name)
            .with_for_trait("enc::DecodeFromReadPrefix")
            .with_function(Function::from(signature).with_statement(fn_source))
    }
}
