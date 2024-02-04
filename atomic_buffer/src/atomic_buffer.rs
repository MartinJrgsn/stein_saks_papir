use std::{sync::{Arc, PoisonError, Mutex, MutexGuard}, collections::VecDeque};

use crate::AtomicBufferWeak;

#[derive(Debug)]
pub struct AtomicBuffer<T>(pub(super) Arc<Mutex<VecDeque<T>>>);

impl<T> AtomicBuffer<T>
{
    pub fn new() -> Self
    {
        Self(Arc::new(Mutex::new(VecDeque::new())))
    }

    #[must_use]
    pub fn push_back(&self, value: T) -> Result<(), PoisonError<MutexGuard<'_, VecDeque<T>>>>
    {
        self.0.lock()?.push_back(value);
        Ok(())
    }

    #[must_use]
    pub fn pop_front(&self) -> Result<Option<T>, PoisonError<MutexGuard<'_, VecDeque<T>>>>
    {
        Ok(self.0.lock()?.pop_front())
    }

    #[must_use]
    pub fn filter_pop_front(&self, filter: impl Fn(&T) -> bool) -> Result<Option<T>, PoisonError<MutexGuard<'_, VecDeque<T>>>>
    {
        let mut vec = self.0.lock()?;

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

    pub fn downgrade(&self) -> AtomicBufferWeak<T>
    {
        AtomicBufferWeak(Arc::downgrade(&self.0))
    }
}

impl<T> Clone for AtomicBuffer<T>
{
    fn clone(&self) -> Self
    {
        Self(self.0.clone())
    }
}
