pub mod try_dyncast_ref;

pub use try_dyncast_ref::*;

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
    From: TryDyncastRef<To> + TryIntoDyncastObj<To, Obj>,
    From::Obj: TryUpcast<To, Obj>,
{
    fn try_dyncast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        self.try_into_dyncast_obj().and_then(|dyn_vee| dyn_vee.try_upcast())
    }
}