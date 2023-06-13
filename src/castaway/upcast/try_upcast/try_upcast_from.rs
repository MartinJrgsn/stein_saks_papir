use super::*;

pub trait TryUpcastFrom<From, Obj>: TryUpcastFromRef<From>
where
    From: Is<Obj> + ?Sized,
    Obj: ?Sized
{
    fn try_upcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>;
}

impl<From, To, Obj> TryUpcastFrom<From, Obj> for To
where
    From: TryUpcast<Self, Obj> + ?Sized,
    To: ?Sized,
    Obj: ?Sized
{
    fn try_upcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>
    {
        from.try_upcast()
    }
}