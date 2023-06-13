pub mod upcast_from;
pub mod try_upcast;
pub mod as_any;

pub use upcast_from::*;
pub use try_upcast::*;
pub use as_any::*;

use std::{marker::Unsize, ops::CoerceUnsized};

use super::*;

pub trait Upcast<To>: TryUpcastRef<To>
where To: ?Sized
{
    fn upcast_ref(self: &Self) -> &To;
    fn upcast_mut(self: &mut Self) -> &mut To;
    fn upcast(self: Box<Self>) -> Box<To>;
}
impl<From, To> Upcast<To> for From
where
    From: Is<To> + ?Sized,
    To: ?Sized
{
    fn upcast_ref(self: &Self) -> &To
    {
        self
    }
    fn upcast_mut(self: &mut Self) -> &mut To
    {
        self
    }
    fn upcast(self: Box<Self>) -> Box<To>
    {
        self
    }
}