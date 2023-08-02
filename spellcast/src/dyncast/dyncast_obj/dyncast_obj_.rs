use super::*;

pub trait DyncastObj<From, To>: Is<From> + Is<To>
where
    From: ?Sized,
    To: ?Sized
{
    type Obj: IsObjOf<Self> + Is<From> + Is<To> + ?Sized;
}