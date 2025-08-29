pub use packet::*;

mod packet;

pub mod io;

const _: () = debug_assert!(usize::BITS == 32 || usize::BITS == 64);
