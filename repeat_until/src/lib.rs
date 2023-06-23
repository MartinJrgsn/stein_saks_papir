#![feature(tuple_trait)]
#![feature(try_trait_v2)]
#![feature(unboxed_closures)]
#![feature(associated_type_bounds)]
#![feature(fn_traits)]

pub mod repeat_until;
pub mod error;
mod tests;

pub use repeat_until::*;
pub use error::*;