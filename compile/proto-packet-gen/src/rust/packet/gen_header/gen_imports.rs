use crate::rust::GenRust;
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet::PacketType;

impl GenRust {
    //! Gen Imports

    /// Generates the imports for the `packet_type`.
    pub(in crate::rust) fn gen_imports(&self, packet_type: PacketType) -> Source {
        let mut source: Source = Source::default()
            .with_semi(format!("use proto_packet::{{Packet, {}}}", packet_type))
            .with_semi("use proto_packet::io::WireType")
            .with_semi("use std::io::{Read, Write}")
            .with_semi("use enc::{EncodedLen, EncodeToSlice, EncodeToWrite}")
            .with_semi("use enc::{DecodeFromRead, DecodeFromReadPrefix}")
            .with_semi("use enc::{Error, StreamError}")
            .with_semi("use serde::{Serialize, Deserialize}");

        if packet_type == PacketType::Enum || packet_type == PacketType::Variant {
            source.add_semi("use proto_packet::io::WithTagNumber");
        }

        source.with_statement(EmptyLine::default())
    }
}
