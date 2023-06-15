use crate::{downcast::Downcast, upcast::Upcast};

use super::*;

pub trait Veecast<To, Struct, Obj>: VeecastRef<To, Struct>
where
    Struct: Is<To>,
    To: ?Sized
{
    fn veecast_ref(self: &Self) -> Option<&To>;
    fn veecast_mut(self: &mut Self) -> Option<&mut To>;
    fn veecast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>;
}
impl<From, To, Struct, Obj> Veecast<To, Struct, Obj> for From
where
    From: Downcast<Struct, Obj> + ?Sized,
    Struct: Upcast<To> + 'static,
    To: ?Sized
{
    fn veecast_ref(self: &Self) -> Option<&To>
    {
        self.downcast_ref().map(|vee| vee.upcast_ref())
    }
    fn veecast_mut(self: &mut Self) -> Option<&mut To>
    {
        self.downcast_mut().map(|vee| vee.upcast_mut())
    }
    fn veecast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        self.downcast().map(|vee| vee.upcast())
    }
}