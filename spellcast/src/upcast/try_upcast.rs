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