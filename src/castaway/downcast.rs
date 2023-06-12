use super::*;

pub trait Downcast<To, Alt>: DowncastRef<To>
where
    To: Is<Self> + ?Sized,
    Alt: ?Sized
{
    fn downcast(self: Box<Self>) -> Result<Box<To>, Box<Alt>>;
}
default impl<From, To, Alt> Downcast<To, Alt> for From
where
    From: Upcast<Alt> + ?Sized,
    To: Is<Self> + DowncastFrom<From> + ?Sized,
    Alt: ?Sized
{
    fn downcast(self: Box<Self>) -> Result<Box<To>, Box<Alt>>
    {
        To::downcast_from(self).map_err(|alt| alt.upcast())
    }
}