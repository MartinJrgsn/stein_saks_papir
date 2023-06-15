use crate::Is;

use super::*;

pub trait Downcast<To, Obj>: DowncastRef<To> + Is<Obj>
where
    To: ?Sized,
    Obj: ?Sized
{
    fn downcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>;
}
impl<'a, From, To, Obj> Downcast<To, Obj> for From
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