use crate::io::WithTagNumber;
use crate::Packet;

/// A variant.
pub trait Variant: Packet + WithTagNumber {}
