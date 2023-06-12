use super::*;

/*pub trait Dyncast<To, Alt>
where
    To: ?Sized,
    Alt: ?Sized
{
    fn dyndowncast_ref(self: &Self) -> Option<&dyn TryUpcast<To, Alt>>;
    fn dyndowncast_mut(self: &mut Self) -> Option<&mut dyn TryUpcast<To, Alt>>;
    fn dyndowncast(self: Box<Self>) -> Result<Box<dyn TryUpcast<To, Alt>>, Box<Alt>>;
    fn dyncast_ref(self: &Self) -> Option<&To>;
    fn dyncast_mut(self: &mut Self) -> Option<&mut To>;
    fn dyncast(self: Box<Self>) -> Result<Box<To>, Box<Alt>>;
}
default impl<From, To, Alt> Dyncast<To, Alt> for From
where
    From: ?Sized,
    To: ?Sized,
    Alt: ?Sized
{
    fn dyncast_ref(self: &Self) -> Option<&To>
    {
        self.dyndowncast_ref().and_then(|value| value.try_upcast_ref())
    }
    fn dyncast_mut(self: &mut Self) -> Option<&mut To>
    {
        self.dyndowncast_mut().and_then(|value| value.try_upcast_mut())
    }
    fn dyncast(self: Box<Self>) -> Result<Box<To>, Box<Alt>>
    {
        self.dyndowncast().and_then(|value| value.try_upcast())
    }
}*/