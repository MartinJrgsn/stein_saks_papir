use crate::{Is, AsAny};

use super::*;

pub trait DowncastFrom<From, Obj>: DowncastFromRef<From>
where
    From: Is<Obj> + ?Sized,
    Obj: ?Sized
{
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>;
}
impl<From, To, Obj> DowncastFrom<From, Obj> for To
where
    From: Is<Obj> + AsAny + ?Sized + 'static,
    To: DowncastFromRef<From> + 'static,
    Obj: ?Sized
{
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>
    {
        if !from.as_any().is::<To>()
        {
            return Ok(from.into_any().downcast().unwrap());
        }
        return Err(from)
    }
}
/*default impl<From, To, Obj> DowncastFrom<From, Obj> for To
where
    From: Is<Obj> + AsAny + ?Sized,
    To: DowncastFromRef<From> + IsImplOf<From> + ?Sized,
    Obj: ?Sized
{
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>
    {
        from.try_dyncast()
    }
}*/