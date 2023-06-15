use super::*;

pub trait DowncastFromRef<From>: Is<From>
where
    From: ?Sized,
{
    fn downcast_from_ref<'a>(from: &'a From) -> Option<&'a Self>;
    fn downcast_from_mut<'a>(from: &'a mut From) -> Option<&'a mut Self>;
}
impl<From, To> DowncastFromRef<From> for To
where
    From: AsAny + ?Sized,
    To: Is<From> + 'static
{
    fn downcast_from_ref<'a>(from: &'a From) -> Option<&'a Self>
    {
        from.as_any().downcast_ref()
    }
    fn downcast_from_mut<'a>(from: &'a mut From) -> Option<&'a mut Self>
    {
        from.as_any_mut().downcast_mut()
    }
}