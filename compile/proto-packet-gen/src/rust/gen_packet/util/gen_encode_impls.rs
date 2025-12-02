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
    pub(in crate::rust::gen_packet) fn gen_encoded_len_impl(
        &self,
        type_name: TypeNameRef,
        fn_source: Source,
    ) -> ImplBlock {
        let signature: Signature = Signature::from("encoded_len")
            .with_receiver(Borrowed)
            .with_result(
                RustType::from("Result")
                    .with_generic(RustPrimitive::UnsignedIntSize)
                    .with_generic("enc::Error"),
            );
        ImplBlock::from(type_name)
            .with_for_trait("enc::EncodedLen")
            .with_function(Function::from(signature).with_statement(fn_source))
    }
}

impl GenRust {
    //! impl `EncodeToSlice`

    /// Generates the impl block for the `EncodeToSlice` trait.
    pub(in crate::rust::gen_packet) fn gen_encode_to_slice_impl(
        &self,
        type_name: TypeNameRef,
        unused_target_param: bool,
        fn_source: Source,
    ) -> ImplBlock {
        let signature: Signature = Signature::from("encode_to_slice_unchecked")
            .with_unsafe()
            .with_receiver(Borrowed)
            .with_param((
                if unused_target_param {
                    "_target"
                } else {
                    "target"
                },
                RustPrimitive::UnsignedInt8
                    .to_type_tag()
                    .to_slice()
                    .to_ref(Reference::default().with_mut()),
            ))
            .with_result(
                RustType::from("Result")
                    .with_generic(RustPrimitive::UnsignedIntSize)
                    .with_generic("enc::Error"),
            );
        ImplBlock::from(type_name)
            .with_for_trait("enc::EncodeToSlice")
            .with_function(Function::from(signature).with_statement(fn_source))
    }
}

impl GenRust {
    //! impl `EncodeToWrite`

    /// Generates the impl block for the `EncodeToWrite` trait.
    pub(in crate::rust::gen_packet) fn gen_encode_to_write_impl(
        &self,
        type_name: TypeNameRef,
        unused_write_param: bool,
        fn_source: Source,
    ) -> ImplBlock {
        let signature: Signature = Signature::from("encode_to_write")
            .with_receiver(Borrowed)
            .with_generic(("W", RustType::from("std::io::Write")))
            .with_param((
                if unused_write_param { "_w" } else { "w" },
                RustType::from("W").to_ref(Reference::default().with_mut()),
            ))
            .with_result(
                RustType::from("Result")
                    .with_generic(RustPrimitive::UnsignedIntSize)
                    .with_generic("enc::Error"),
            );
        ImplBlock::from(type_name)
            .with_for_trait("enc::EncodeToWrite")
            .with_function(Function::from(signature).with_statement(fn_source))
    }
}
