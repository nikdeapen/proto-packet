use crate::Packet;
use crate::io::WithTagNumber;

/// An enum.
pub trait Enum: Packet + WithTagNumber {}
