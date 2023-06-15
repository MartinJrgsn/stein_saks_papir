use crate::upcast::TryUpcast;

use super::*;

pub trait TryIntoDyncastObj<From, To, Alt>: TryAsDyncastObj<From, To> + DyncastObj<From, To, Obj: TryUpcast<To, Alt>> + Is<Alt>
where
    From: ?Sized,
    To: ?Sized,
    Alt: ?Sized
{
    fn try_into_dyncast_obj(self: Box<Self>) -> Result<Box<Self::Obj>, Box<Alt>>;
}
impl<Medium, From, To, Alt> TryIntoDyncastObj<From, To, Alt> for Medium
where
    Medium: TryUpcast<Self::Obj, Alt> + TryAsDyncastObj<From, To> + DyncastObj<From, To, Obj: TryUpcast<To, Alt>> + Is<Alt> + ?Sized,
    From: ?Sized,
    To: ?Sized,
    Alt: ?Sized
{
    fn try_into_dyncast_obj(self: Box<Self>) -> Result<Box<Self::Obj>, Box<Alt>>
    {
        self.try_upcast()
    }
}