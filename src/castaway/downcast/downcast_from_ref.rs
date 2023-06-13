use super::*;

pub trait DowncastFromRef<From>: Is<From>
where
    From: ?Sized,
{
    fn downcast_from_ref(from: &From) -> Option<&Self>;
    fn downcast_from_mut(from: &mut From) -> Option<&mut Self>;
}
default impl<From, To> DowncastFromRef<From> for To
where
    From: AsAny + ?Sized,
    To: Is<From>
{
    fn downcast_from_ref(from: &From) -> Option<&Self>
    {
        from.as_any().downcast_ref()
    }
    fn downcast_from_mut(from: &mut From) -> Option<&mut Self>
    {
        from.as_any_mut().downcast_mut()
    }
}
impl<From, To> DowncastFromRef<From> for To
where
    From: AsAny + ?Sized + TryDyncastRef<To>,
    To: Is<From> + Unsized
{
    fn downcast_from_ref(from: &From) -> Option<&Self>
    {
        from.try_dyncast_ref()
    }
    fn downcast_from_mut(from: &mut From) -> Option<&mut Self>
    {
        from.try_dyncast_mut()
    }
}