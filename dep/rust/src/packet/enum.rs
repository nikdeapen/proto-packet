use crate::io::WithTagNumber;
use crate::Packet;

/// An enum.
pub trait Enum: Packet + WithTagNumber {}
