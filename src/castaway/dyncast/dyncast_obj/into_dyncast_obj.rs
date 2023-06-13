use super::*;

pub trait IntoDyncastObj<To>: AsDyncastObj<To>
where
    To: ?Sized
{
    fn into_dyncast_obj(self: Box<Self>) -> Box<Self::Obj>;
}
impl<From, To> IntoDyncastObj<To> for From
where
    From: DyncastObj<To, Obj: Upcast<To>> + Upcast<Self::Obj> + AsDyncastObj<To> + TryIntoDyncastObj<To, Self::Obj> + ?Sized,
    To: ?Sized
{
    fn into_dyncast_obj(self: Box<Self>) -> Box<Self::Obj>
    {
        self.upcast()
    }
}