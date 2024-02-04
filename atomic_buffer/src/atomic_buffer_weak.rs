use std::{sync::{Weak, Mutex, Arc}, collections::VecDeque};

use crate::error::BufferError;

#[derive(Debug)]
pub struct AtomicBufferWeak<T>(pub(super) Weak<Mutex<VecDeque<T>>>);

impl<T> AtomicBufferWeak<T>
{
    #[must_use]
    fn arc(&self) -> Result<Arc<Mutex<VecDeque<T>>>, BufferError>
    {
        Ok(self.0.upgrade().ok_or_else(|| BufferError::NullPointerError)?)
    }

    #[must_use]
    pub fn push_back(&self, value: T) -> Result<(), BufferError>
    {
        Ok(self.arc()?.lock()?.push_back(value))
    }

    #[must_use]
    pub fn pop_front(&self) -> Result<Option<T>, BufferError>
    {
        Ok(self.arc()?.lock()?.pop_front())
    }

    #[must_use]
    pub fn filter_pop_front(&self, filter: impl Fn(&T) -> bool) -> Result<Option<T>, BufferError>
    {
        let arc = self.arc()?;
        let mut vec = arc.lock()?;

        for (n, e) in vec.iter()
            .enumerate()
        {
            if filter(e)
            {
                return Ok(vec.remove(n))
            }
        }

        Ok(None)
    }
}
impl<T> Clone for AtomicBufferWeak<T>
{
    fn clone(&self) -> Self
    {
        Self(self.0.clone())
    }
}
