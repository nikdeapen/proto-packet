use crate::rust::EncodeOp::{EncodeToSlice, EncodeToWrite, EncodedLen};
use crate::rust::{EncodeOp, GenRust};
use code_gen::{Source, WithStatements};
use proto_packet_tree::{TypeNameRef, WithTypeName};

impl GenRust {
    //! Gen Encode

    /// Generates the encoding impl blocks for the `element`.
    pub(in crate::rust::gen_packet) fn gen_encode<T, F>(
        &self,
        element: &T,
        is_empty: bool,
        gen_source_fn: F,
    ) -> Source
    where
        T: WithTypeName,
        F: Fn(&T, EncodeOp) -> Source,
    {
        let name: TypeNameRef = element.type_name();
        Source::default()
            .with_empty_line()
            .with_statement(self.gen_encoded_len_impl(name, gen_source_fn(element, EncodedLen)))
            .with_empty_line()
            .with_statement(self.gen_encode_to_slice_impl(
                name,
                is_empty,
                gen_source_fn(element, EncodeToSlice),
            ))
            .with_empty_line()
            .with_statement(self.gen_encode_to_write_impl(
                name,
                is_empty,
                gen_source_fn(element, EncodeToWrite),
            ))
    }
}
