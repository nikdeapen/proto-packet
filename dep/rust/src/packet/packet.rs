use crate::io::WireType;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
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
{
    /// Gets the wire type.
    fn wire_type() -> WireType;
}
