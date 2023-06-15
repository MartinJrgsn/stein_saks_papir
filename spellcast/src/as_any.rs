use std::{any::Any, marker::Unsize};

pub trait AsAny: Any + Unsize<dyn Any>
{
    fn as_any<'a>(self: &'a Self) -> &'a dyn Any
    {
        self
    }
    fn as_any_mut<'a>(self: &'a mut Self) -> &'a mut dyn Any
    {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any>
    {
        self
    }
}

impl<T> AsAny for T
where T: Any + Unsize<dyn Any> + ?Sized {}