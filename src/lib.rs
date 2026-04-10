#![allow(clippy::module_inception)]

pub use packet::*;

mod packet;

pub mod float;
pub mod io;

mod macros;

#[cfg(feature = "serde")]
pub mod service;
