use std::fmt::Debug;
use std::hash::Hash;

/// A packet of data.
pub trait Packet: Clone + Ord + PartialOrd + Eq + PartialEq + Hash + Debug {}
