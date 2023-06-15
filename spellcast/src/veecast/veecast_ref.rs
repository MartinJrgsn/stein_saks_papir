use crate::{downcast::DowncastRef, upcast::Upcast};

use super::*;

pub trait VeecastRef<To, Struct>: IsObjOf<Struct>
where
    Struct: Is<To>,
    To: ?Sized
{
    fn veecast_ref(self: &Self) -> Option<&To>;
    fn veecast_mut(self: &mut Self) -> Option<&mut To>;
}
impl<From, To, Struct> VeecastRef<To, Struct> for From
where
    From: DowncastRef<Struct> + ?Sized,
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
}