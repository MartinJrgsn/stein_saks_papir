use super::*;

pub trait AsDyncastObj<To>: TryAsDyncastObj<To> + DyncastObj<To, Obj: Upcast<To>>
where
    To: ?Sized
{
    fn as_dyncast_obj(self: &Self) -> &Self::Obj;
    fn as_dyncast_obj_mut(self: &mut Self) -> &mut Self::Obj;
}
impl<From, To> AsDyncastObj<To> for From
where
    From: DyncastObj<To, Obj: Upcast<To>> + Upcast<Self::Obj> + TryAsDyncastObj<To> + ?Sized,
    To: ?Sized
{
    fn as_dyncast_obj(self: &Self) -> &Self::Obj
    {
        self.upcast_ref()
    }
    fn as_dyncast_obj_mut(self: &mut Self) -> &mut Self::Obj
    {
        self.upcast_mut()
    }
}