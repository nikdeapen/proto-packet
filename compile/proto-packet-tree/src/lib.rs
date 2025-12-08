#![allow(clippy::module_inception)]

pub use common::*;
pub use message::*;
pub use naming::*;
pub use r#enum::*;
pub use r#struct::*;
pub use schema::*;
pub use var::*;
pub use variant::*;

mod common;
mod r#enum;
mod message;
mod naming;
mod schema;
mod r#struct;
mod var;
mod variant;
