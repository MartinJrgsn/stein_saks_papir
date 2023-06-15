use crate::upcast::TryUpcast;

use super::*;

pub trait TryDyncast<To, Obj>: Is<Obj> + TryDyncastRef<To>
where
    To: ?Sized,
    Obj: ?Sized
{
    fn try_dyncast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>;
}
impl<From, To, Obj> TryDyncast<To, Obj> for From
where
    From: TryDyncastRef<To> + TryIntoDyncastObj<From, To, Obj> + ?Sized,
    From::Obj: TryUpcast<To, Obj>,
    To: ?Sized
{
    fn try_dyncast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        self.try_into_dyncast_obj().and_then(|dyn_vee| dyn_vee.try_upcast())
    }
}