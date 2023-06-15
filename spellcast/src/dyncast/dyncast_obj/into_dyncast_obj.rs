use crate::upcast::Upcast;

use super::*;

pub trait IntoDyncastObj<From, To>: AsDyncastObj<From, To>
where
    From: ?Sized,
    To: ?Sized
{
    fn into_dyncast_obj(self: Box<Self>) -> Box<Self::Obj>;
}
impl<Medium, From, To> IntoDyncastObj<From, To> for Medium
where
    Medium: DyncastObj<From, To, Obj: Upcast<To>> + Upcast<Self::Obj> + AsDyncastObj<From, To> + TryIntoDyncastObj<From, To, From> + ?Sized,
    From: ?Sized,
    To: ?Sized
{
    fn into_dyncast_obj(self: Box<Self>) -> Box<Self::Obj>
    {
        self.upcast()
    }
}