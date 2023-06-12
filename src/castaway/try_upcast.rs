use std::marker::Unsize;

use super::*;

pub trait TryUpcast<To, Obj>: Is<Obj> + Unsize<Obj>
where
    To: ?Sized,
    Obj: ?Sized
{
    fn is_of(self: &Self) -> bool
    {
        false
    }
    fn try_upcast_ref(self: &Self) -> Option<&To>
    {
        None
    }
    fn try_upcast_mut(self: &mut Self) -> Option<&mut To>
    {
        None
    }
    fn try_upcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        Err(self)
    }
}
impl<From, To, Obj> TryUpcast<To, Obj> for From
where
    From: Is<To> + Is<Obj> + Unsize<Obj> + Upcast<To>,
    To: Downcast<From, Obj> + ?Sized
{
    fn is_of(self: &Self) -> bool
    {
        true
    }
    fn try_upcast_ref(self: &Self) -> Option<&To>
    {
        Some(self.upcast_ref())
    }
    fn try_upcast_mut(self: &mut Self) -> Option<&mut To>
    {
        Some(self.upcast_mut())
    }
    fn try_upcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        Ok(self.upcast())
    }
}