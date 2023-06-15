use crate::upcast::Upcast;

use super::*;

pub trait DyncastRef<To>
where
    To: ?Sized
{
    fn dyncast_ref(self: &Self) -> &To;
    fn dyncast_mut(self: &mut Self) -> &mut To;
}
impl<From, To> DyncastRef<To> for From
where
    From: IntoDyncastObj<From, To> + ?Sized,
    To: ?Sized
{
    fn dyncast_ref(self: &Self) -> &To
    {
        self.as_dyncast_obj().upcast_ref()
    }
    fn dyncast_mut(self: &mut Self) -> &mut To
    {
        self.as_dyncast_obj_mut().upcast_mut()
    }
}