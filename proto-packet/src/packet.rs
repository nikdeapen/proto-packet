use std::fmt::{Debug, Display};
use std::hash::Hash;

use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

/// A packet of data.
pub trait Packet:
    Clone
    + Ord
    + PartialOrd
    + Eq
    + PartialEq
    + Hash
    + Debug
    + Display
    + Sync
    + Sized
    + EncodedLen
    + EncodeToSlice
    + EncodeToWrite
{
}
