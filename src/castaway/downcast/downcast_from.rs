use super::*;

pub trait DowncastFrom<From, Obj>: DowncastFromRef<From>
where
    From: Is<Obj> + ?Sized,
    Obj: ?Sized
{
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>;
}
default impl<From, To, Obj> DowncastFrom<From, Obj> for To
where
    From: Is<Obj> + AsAny + ?Sized,
    To: DowncastFromRef<From> + IsImplOf<From>,
    Obj: ?Sized
{
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>
    {
        if !Self::is(&*from)
        {
            return Ok(from.into_any().downcast().unwrap());
        }
        return Err(from)
    }
}
impl<From, To, Obj> DowncastFrom<From, Obj> for To
where
    From: Is<Obj> + AsAny + ?Sized + TryDyncast<To, Obj>,
    To: DowncastFromRef<From> + IsImplOf<From> + Unsized,
    Obj: ?Sized
{
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>
    {
        from.try_dyncast()
    }
}