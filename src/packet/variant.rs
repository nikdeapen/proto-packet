use crate::Packet;
use crate::io::WithTagNumber;

/// A variant.
pub trait Variant: Packet + WithTagNumber {}
