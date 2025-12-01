use crate::WireType;
use enc::{DecodeFromRead, DecodeFromReadPrefix};
use std::fmt::Debug;
use std::hash::Hash;

/// A packet of data.
pub trait Packet:
    Clone + Ord + PartialOrd + Eq + PartialEq + Hash + Debug + DecodeFromRead + DecodeFromReadPrefix
{
    /// Gets the wire type.
    fn wire_type() -> WireType;
}
