use crate::upcast::Upcast;

use super::*;

pub trait AsDyncastObj<From, To>: TryAsDyncastObj<From, To> + DyncastObj<From, To, Obj: Upcast<To>>
where
    From: ?Sized,
    To: ?Sized
{
    fn as_dyncast_obj(self: &Self) -> &Self::Obj;
    fn as_dyncast_obj_mut(self: &mut Self) -> &mut Self::Obj;
}
impl<Medium, From, To> AsDyncastObj<From, To> for Medium
where
    Medium: DyncastObj<From, To, Obj: Upcast<To>> + TryAsDyncastObj<From, To> + Upcast<Self::Obj> + ?Sized,
    From: ?Sized,
    To: ?Sized
{
    fn as_dyncast_obj(self: &Self) -> &Self::Obj
    {
        self.upcast_ref()
    }
    fn as_dyncast_obj_mut(self: &mut Self) -> &mut Self::Obj
    {
        self.upcast_mut()
    }
}