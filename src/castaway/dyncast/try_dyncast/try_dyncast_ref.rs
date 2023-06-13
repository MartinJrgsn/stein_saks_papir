use super::*;

pub trait TryDyncastRef<To>: TryAsDyncastObj<To>
where
    To: ?Sized
{
    fn try_dyncast_ref(self: &Self) -> Option<&To>;
    fn try_dyncast_mut(self: &mut Self) -> Option<&mut To>;
}
impl<From, To> TryDyncastRef<To> for From
where
    From: TryAsDyncastObj<To> + ?Sized,
    To: ?Sized,
    dyn TryUpcastRef<To>: TryUpcastRef<To>
{
    fn try_dyncast_ref(self: &Self) -> Option<&To>
    {
        self.try_as_dyncast_obj().and_then(|dyn_vee| dyn_vee.try_upcast_ref())
    }
    fn try_dyncast_mut(self: &mut Self) -> Option<&mut To>
    {
        self.try_as_dyncast_obj_mut().and_then(|dyn_vee| dyn_vee.try_upcast_mut())
    }
}