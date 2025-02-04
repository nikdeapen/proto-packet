use code_gen::{EmptyLine, Source, WithStatements};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Variant Imports

    /// Generates the imports for the `Variant` source code.
    pub(in crate::rust::variant) fn gen_variant_imports(&self) -> Source {
        Source::default()
            .with_semi("use proto_packet::{Packet, Variant}")
            .with_semi("use proto_packet::io::{WireType}")
            .with_semi("use enc::{EncodedLen, EncodeToSlice, EncodeToWrite}")
            .with_semi("use enc::{DecodeFromRead, DecodeFromReadPrefix}")
            .with_semi("use std::io::{Read, Write, Error}")
            .with_statement(EmptyLine::default())
    }
}
