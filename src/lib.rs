#![allow(clippy::module_inception)]

pub use packet::*;

mod packet;

pub mod float;
pub mod io;

#[cfg(feature = "serde")]
pub mod service;
