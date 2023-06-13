pub mod downcast_from_ref;
pub mod downcast_from;
pub mod downcast_ref;

pub use downcast_from_ref::*;
pub use downcast_from::*;
pub use downcast_ref::*;

use super::*;

pub trait Downcast<To, Obj>: DowncastRef<To> + Is<Obj>
where
    To: ?Sized,
    Obj: ?Sized
{
    fn downcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>;
}
impl<From, To, Obj> Downcast<To, Obj> for From
where
    From: DowncastRef<To> + Is<Obj> + ?Sized,
    To: DowncastFrom<Self, Obj> + ?Sized,
    Obj: ?Sized
{
    fn downcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        To::downcast_from(self)
    }
}