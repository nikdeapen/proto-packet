use crate::io::WireType;
use std::fmt::Debug;
use std::hash::Hash;

/// A packet.
pub trait Packet: Clone + Ord + PartialOrd + Eq + PartialEq + Hash + Debug {
    /// Gets the wire type.
    fn wire() -> WireType;
}
