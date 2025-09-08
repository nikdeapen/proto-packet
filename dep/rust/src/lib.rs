pub use packet::*;

mod packet;

pub mod io;

// todo -- list usize bit dependencies
const _: () = debug_assert!(usize::BITS <= u64::BITS);
