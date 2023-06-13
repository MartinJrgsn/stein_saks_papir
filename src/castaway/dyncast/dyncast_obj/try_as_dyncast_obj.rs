use super::*;

pub trait TryAsDyncastObj<To>: DyncastObj<To, Obj: TryUpcastRef<To>>
where
    To: ?Sized
{
    fn try_as_dyncast_obj(self: &Self) -> Option<&Self::Obj>;
    fn try_as_dyncast_obj_mut(self: &mut Self) -> Option<&mut Self::Obj>;
}
default impl<From, To> TryAsDyncastObj<To> for From
where
    From: TryUpcastRef<Self::Obj> + DyncastObj<To, Obj: TryUpcastRef<To>> + ?Sized,
    To: ?Sized
{
    fn try_as_dyncast_obj(self: &Self) -> Option<&Self::Obj>
    {
        self.try_upcast_ref()
    }
    fn try_as_dyncast_obj_mut(self: &mut Self) -> Option<&mut Self::Obj>
    {
        self.try_upcast_mut()
    }
}