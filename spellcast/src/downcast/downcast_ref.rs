use crate::IsObjOf;

use super::*;

pub trait DowncastRef<To>: IsObjOf<To>
where
    To: ?Sized
{
    fn downcast_ref(self: &Self) -> Option<&To>;
    fn downcast_mut(self: &mut Self) -> Option<&mut To>;
}
impl<'a, From, To> DowncastRef<To> for From
where
    From: ?Sized,
    To: DowncastFromRef<From> + ?Sized
{
    fn downcast_ref(self: &Self) -> Option<&To>
    {
        To::downcast_from_ref(self)
    }
    fn downcast_mut(self: &mut Self) -> Option<&mut To>
    {
        To::downcast_from_mut(self)
    }
}