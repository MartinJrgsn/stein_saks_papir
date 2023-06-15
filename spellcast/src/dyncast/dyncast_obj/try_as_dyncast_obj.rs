use crate::upcast::TryUpcastRef;

use super::*;

pub trait TryAsDyncastObj<From, To>: DyncastObj<From, To, Obj: TryUpcastRef<To>>
where
    From: ?Sized,
    To: ?Sized
{
    fn try_as_dyncast_obj(self: &Self) -> Option<&Self::Obj>;
    fn try_as_dyncast_obj_mut(self: &mut Self) -> Option<&mut Self::Obj>;
}
impl<Medium, From, To> TryAsDyncastObj<From, To> for Medium
where
    Medium: TryUpcastRef<Self::Obj> + DyncastObj<From, To, Obj: TryUpcastRef<To>> + ?Sized,
    From: ?Sized,
    To: ?Sized
{
    fn try_as_dyncast_obj(self: &Self) -> Option<&Self::Obj>
    {
        self.try_upcast_ref()
    }
    fn try_as_dyncast_obj_mut(self: &mut Self) -> Option<&mut Self::Obj>
    {
        self.try_upcast_mut()
    }
}