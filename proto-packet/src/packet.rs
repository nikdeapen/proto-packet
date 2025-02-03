use std::fmt::Debug;
use std::hash::Hash;

use crate::io::WireType;

// A packet.
pub trait Packet: Clone + Ord + PartialOrd + Eq + PartialEq + Hash + Debug {
    /// Gets the wire type.
    fn wire_type() -> WireType;
}
