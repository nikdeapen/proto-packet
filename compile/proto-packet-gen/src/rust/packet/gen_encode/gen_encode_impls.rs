use crate::rust::GenRust;
use code_gen::rust::Receiver::Borrowed;
use code_gen::rust::{
    Function, ImplBlock, Reference, RustPrimitive, RustType, Signature, WithFnGenerics,
    WithFunctions, WithReceiver, WithResult, WithUnsafeFlag, WithVarParams,
};
use code_gen::{Source, WithStatements};
use proto_packet_tree::TypeNameRef;

impl GenRust {
    //! impl `EncodedLen`

    /// Generates the impl block for the `EncodedLen` trait.
    pub(in crate::rust) fn gen_encoded_len_impl(
        &self,
        type_name: TypeNameRef,
        fn_source: Source,
    ) -> ImplBlock {
        let signature: Signature = Signature::from("encoded_len")
            .with_receiver(Borrowed)
            .with_result(
                RustType::from("Result")
                    .with_generic(RustPrimitive::UnsignedIntSize)
                    .with_generic("Error"),
            );
        ImplBlock::from(type_name)
            .with_for_trait("EncodedLen")
            .with_function(Function::from(signature).with_statement(fn_source))
    }
}

impl GenRust {
    //! impl `EncodeToSlice`

    pub(in crate::rust) fn gen_encode_to_slice_impl(
        &self,
        type_name: TypeNameRef,
        fn_source: Source,
    ) -> ImplBlock {
        let signature: Signature = Signature::from("encode_to_slice_unchecked")
            .with_unsafe()
            .with_receiver(Borrowed)
            .with_param((
                "target",
                RustPrimitive::UnsignedInt8
                    .to_type_tag()
                    .to_slice()
                    .to_ref(Reference::default().with_mut()),
            ))
            .with_result(
                RustType::from("Result")
                    .with_generic(RustPrimitive::UnsignedIntSize)
                    .with_generic("Error"),
            );
        ImplBlock::from(type_name)
            .with_for_trait("EncodeToSlice")
            .with_function(Function::from(signature).with_statement(fn_source))
    }
}

impl GenRust {
    //! impl `EncodeToWrite`

    pub(in crate::rust) fn gen_encode_to_write_impl(
        &self,
        type_name: TypeNameRef,
        fn_source: Source,
    ) -> ImplBlock {
        let signature: Signature = Signature::from("encode_to_write")
            .with_receiver(Borrowed)
            .with_generic(("W", RustType::from("Write")))
            .with_param((
                "w",
                RustType::from("W").to_ref(Reference::default().with_mut()),
            ))
            .with_result(
                RustType::from("Result")
                    .with_generic(RustPrimitive::UnsignedIntSize)
                    .with_generic("StreamError"),
            );
        ImplBlock::from(type_name)
            .with_for_trait("EncodeToWrite")
            .with_function(Function::from(signature).with_statement(fn_source))
    }
}
