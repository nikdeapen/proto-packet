use crate::io::WireType;
use crate::PacketType;
use enc::{DecodeFromRead, DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};
use std::fmt::Debug;
use std::hash::Hash;

/// A packet.
pub trait Packet:
    Clone
    + Ord
    + PartialOrd
    + Eq
    + PartialEq
    + Hash
    + Debug
    + EncodedLen
    + EncodeToSlice
    + EncodeToWrite
    + DecodeFromRead
    + DecodeFromReadPrefix
{
    /// Gets the wire type.
    fn wire_type() -> WireType;

    /// Gets the packet type.
    fn packet_type() -> PacketType;
}
