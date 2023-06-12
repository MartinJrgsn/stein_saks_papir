use std::marker::Unsize;

use super::*;

pub trait Upcast<To>: Is<To>
where To: ?Sized
{
    fn upcast_ref(self: &Self) -> &To;
    fn upcast_mut(self: &mut Self) -> &mut To;
    fn upcast(self: Box<Self>) -> Box<To>;
}
impl<From, To> Upcast<To> for From
where
    From: Is<To> + Unsize<To> + ?Sized,
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