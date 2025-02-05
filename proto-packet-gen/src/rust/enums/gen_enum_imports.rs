use code_gen::{EmptyLine, Source, WithStatements};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Enum Imports

    /// Generates the imports for the `Enum` source code.
    pub(in crate::rust::enums) fn gen_enum_imports(&self) -> Source {
        Source::default()
            .with_semi("use proto_packet::{Packet, Enum}")
            .with_semi("use proto_packet::io::{WireType}")
            .with_semi("use enc::{EncodedLen, EncodeToSlice, EncodeToWrite}")
            .with_semi("use enc::{DecodeFromRead, DecodeFromReadPrefix}")
            .with_semi("use std::io::{Read, Write, Error}")
            .with_semi("use std::str::FromStr")
            .with_statement(EmptyLine::default())
    }
}
