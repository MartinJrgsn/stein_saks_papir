use std::any::Any;

use super::*;

pub trait AsAny: Any + Upcast<dyn Any>
{
    fn as_any(self: &Self) -> &dyn Any
    {
        self.upcast_ref()
    }
    fn as_any_mut(self: &mut Self) -> &mut dyn Any
    {
        <Self as Upcast<dyn Any>>::upcast_mut(self)
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any>
    {
        <Self as Upcast<dyn Any>>::upcast(self)
    }
}
impl<T> AsAny for T
where T: Upcast<dyn Any> + Any + ?Sized {}