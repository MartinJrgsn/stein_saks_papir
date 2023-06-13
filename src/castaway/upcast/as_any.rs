use std::{any::Any, marker::Unsize};

use super::*;

pub trait AsAny: Any + Unsize<dyn Any>
{
    fn as_any(self: &Self) -> &dyn Any
    {
        self
    }
    fn as_any_mut(self: &mut Self) -> &mut dyn Any
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