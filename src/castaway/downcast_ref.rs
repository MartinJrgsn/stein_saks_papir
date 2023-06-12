use super::*;

pub trait DowncastRef<To>
where
    To: Is<Self> + ?Sized
{
    fn downcast_ref(self: &Self) -> Option<&To>;
    fn downcast_mut(self: &mut Self) -> Option<&mut To>;
}
default impl<From, To> DowncastRef<To> for From
where
    From: ?Sized,
    To: DowncastFrom<From> + ?Sized
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