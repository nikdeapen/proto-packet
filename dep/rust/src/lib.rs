pub use packet::*;

mod packet;

pub mod io;

#[cfg(feature = "service")]
pub mod service;

// todo -- list usize bit dependencies
const _: () = debug_assert!(usize::BITS <= u64::BITS);
