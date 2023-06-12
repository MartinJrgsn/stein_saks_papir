use super::*;

pub trait DowncastFrom<From>: Is<From>
where From: ?Sized
{
    fn is(from: &From) -> bool;
    fn downcast_from_ref(from: &From) -> Option<&Self>;
    fn downcast_from_mut(from: &mut From) -> Option<&mut Self>;
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<From>>;
}
impl<From, To> DowncastFrom<From> for To
where
    From: AsAny + ?Sized,
    To: Is<From>
{
    fn is(from: &From) -> bool
    {
        from.as_any().is::<Self>()
    }
    fn downcast_from_ref(from: &From) -> Option<&Self>
    {
        from.as_any().downcast_ref()
    }
    fn downcast_from_mut(from: &mut From) -> Option<&mut Self>
    {
        from.as_any_mut().downcast_mut()
    }
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<From>>
    {
        if !Self::is(&*from)
        {
            return Ok(from.into_any().downcast().unwrap());
        }
        return Err(from)
    }
}