use crate::upcast::Upcast;

use super::*;

pub trait Dyncast<To>: DyncastRef<To>
where
    To: ?Sized
{
    fn dyncast(self: Box<Self>) -> Box<To>;
}
impl<From, To> Dyncast<To> for From
where
    From: DyncastRef<To> + IntoDyncastObj<From, To> + ?Sized,
    To: ?Sized,
{
    fn dyncast(self: Box<Self>) -> Box<To>
    {
        self.into_dyncast_obj().upcast()
    }
}