use super::*;

pub trait UpcastFrom<From>
where
    From: ?Sized
{
    fn upcast_from_ref(from: &From) -> &Self;
    fn upcast_from_mut(from: &mut From) -> &mut Self;
    fn upcast_from(from: Box<From>) -> Box<Self>;
}

impl<From, To> UpcastFrom<From> for To
where
    From: Upcast<To> + ?Sized,
    To: ?Sized
{
    fn upcast_from_ref(from: &From) -> &Self
    {
        from.upcast_ref()
    }
    fn upcast_from_mut(from: &mut From) -> &mut Self
    {
        from.upcast_mut()
    }
    fn upcast_from(from: Box<From>) -> Box<Self>
    {
        from.upcast()
    }
}