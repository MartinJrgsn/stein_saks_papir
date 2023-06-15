use super::*;

pub trait TryUpcastFromRef<From>: IsObjOf<From>
where
    From: ?Sized
{
    fn try_upcast_from_ref(from: &From) -> Option<&Self>;
    fn try_upcast_from_mut(from: &mut From) -> Option<&mut Self>;
}

impl<From, To> TryUpcastFromRef<From> for To
where
    From: TryUpcastRef<To> + ?Sized,
    To: ?Sized
{
    fn try_upcast_from_ref(from: &From) -> Option<&Self>
    {
        from.try_upcast_ref()
    }
    fn try_upcast_from_mut(from: &mut From) -> Option<&mut Self>
    {
        from.try_upcast_mut()
    }
}