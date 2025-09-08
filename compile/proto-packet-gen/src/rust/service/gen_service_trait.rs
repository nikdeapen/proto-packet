use crate::rust::GenRust;
use code_gen::rust::Access::Public;
use code_gen::rust::{
    Receiver, RustType, Signature, SignatureDec, Trait, WithAccess, WithComments as RustComments,
    WithReceiver, WithResult, WithTraitFunctions, WithVarParams,
};
use proto_packet_tree::{Service, ServiceCall, WithComments, WithServiceCallName, WithTypeName};

impl GenRust {
    //! Gen Service: Trait

    /// Generates the trait declaration for the `service`.
    pub(in crate::rust::service) fn gen_service_trait(&self, service: &Service) -> Trait {
        let mut result: Trait = Trait::from(self.naming.type_name(service.type_name()));

        for comment in service.comments() {
            result.add_comment(comment);
        }
        result.set_access(Public);

        for service_call in service.service_calls() {
            result.add_signature_dec(self.gen_service_trait_fn(service_call));
        }

        result
    }

    /// Generates the trait function for the `service_call`.
    fn gen_service_trait_fn(&self, service_call: &ServiceCall) -> SignatureDec {
        let signature: Signature = Signature::from(
            self.naming
                .service_call_name(service_call.service_call_name()),
        )
        .with_receiver(Receiver::Borrowed)
        .with_param(("input", self.typing.owned(service_call.input_type())))
        .with_result(
            RustType::from("Result")
                .with_generic(self.typing.owned(service_call.output_type()))
                .with_generic(RustType::from("()")),
        );

        let mut signature: SignatureDec = SignatureDec::from(signature);
        for comment in service_call.comments() {
            signature.add_comment(comment);
        }

        signature
    }
}
