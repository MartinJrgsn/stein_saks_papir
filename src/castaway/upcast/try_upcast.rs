pub mod try_upcast_from_ref;
pub mod try_upcast_from;
pub mod try_upcast_ref;

pub use try_upcast_from_ref::*;
pub use try_upcast_from::*;
pub use try_upcast_ref::*;

use super::*;

pub trait TryUpcast<To, Obj>: Is<Obj> + TryUpcastRef<To>
where
    To: ?Sized,
    Obj: ?Sized
{
    fn try_upcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        Err(self)
    }
}
impl<From, To, Obj> TryUpcast<To, Obj> for From
where
    From: Is<Obj> + Upcast<To> + ?Sized,
    To: ?Sized,
    Obj: ?Sized
{
    fn try_upcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        Ok(self.upcast())
    }
}