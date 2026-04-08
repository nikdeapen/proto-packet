use crate::io::WireType;
use enc::{DecodeFromRead, DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};
use std::fmt::Debug;
use std::hash::Hash;

/// A packet.
pub trait Packet:
    'static
    + Clone
    + Ord
    + PartialOrd
    + Eq
    + PartialEq
    + Hash
    + Debug
    + Sync
    + Send
    + EncodedLen
    + EncodeToSlice
    + EncodeToWrite
    + DecodeFromRead
    + DecodeFromReadPrefix
{
    /// Gets the wire type.
    fn wire() -> WireType;
}
