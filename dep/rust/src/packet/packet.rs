use crate::io::WireType;
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
    + DecodeFromRead // todo -- do we need both `DecodeFromRead` & `DecodeFromReadPrefix`?
    + DecodeFromReadPrefix
{
    /// Gets the wire type.
    fn wire_type() -> WireType;

    // todo -- packet type fn?
}
