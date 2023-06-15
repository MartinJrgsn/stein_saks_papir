use super::*;

pub trait TryUpcastRef<To>: Is<To>
where
    To: ?Sized
{
    fn is_of(self: &Self) -> bool;
    fn try_upcast_ref(self: &Self) -> Option<&To>;
    fn try_upcast_mut(self: &mut Self) -> Option<&mut To>;
}
impl<From, To> TryUpcastRef<To> for From
where
    From: Upcast<To> + ?Sized,
    To: ?Sized
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
}