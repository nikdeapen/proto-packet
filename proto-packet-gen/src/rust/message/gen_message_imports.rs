use code_gen::{EmptyLine, Source, WithStatements};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Message Imports

    /// Generates the imports for the `Message` source code.
    pub(in crate::rust::message) fn gen_message_imports(&self) -> Source {
        Source::default()
            .with_semi("use std::io::{Error, Write}")
            .with_semi("use proto_packet::{Packet, Message}")
            .with_semi("use proto_packet::io::{WireType, TagNumber}")
            .with_semi("use enc::{EncodedLen, EncodeToSlice, EncodeToWrite}")
            .with_statement(EmptyLine::default())
    }
}
