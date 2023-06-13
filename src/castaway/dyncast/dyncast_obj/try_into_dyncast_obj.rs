use super::*;

pub trait TryIntoDyncastObj<To, Obj>: TryAsDyncastObj<To> + DyncastObj<To, Obj: TryUpcast<To, Obj>> + Is<Obj>
where
    To: ?Sized,
    Obj: ?Sized
{
    fn try_into_dyncast_obj(self: Box<Self>) -> Result<Box<Self::Obj>, Box<Obj>>;
}
impl<From, To, Obj> TryIntoDyncastObj<To, Obj> for From
where
    From: TryUpcast<Self::Obj, Obj> + TryAsDyncastObj<To> + DyncastObj<To, Obj: TryUpcast<To, Obj>> + Is<Obj> + ?Sized,
    To: ?Sized,
    Obj: ?Sized
{
    fn try_into_dyncast_obj(self: Box<Self>) -> Result<Box<Self::Obj>, Box<Obj>>
    {
        self.try_upcast()
    }
}