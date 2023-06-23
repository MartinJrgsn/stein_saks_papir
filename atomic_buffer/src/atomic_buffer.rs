use std::{sync::{Arc, PoisonError, Mutex, MutexGuard}, collections::VecDeque};

use crate::AtomicBufferWeak;

#[derive(Clone)]
pub struct AtomicBuffer<T>(pub(super) Arc<Mutex<VecDeque<T>>>);

impl<T> AtomicBuffer<T>
{
    pub fn new() -> Self
    {
        Self(Arc::new(Mutex::new(VecDeque::new())))
    }

    pub fn push_back(&self, value: T) -> Result<(), PoisonError<MutexGuard<'_, VecDeque<T>>>>
    {
        self.0.lock()?.push_back(value);
        Ok(())
    }

    pub fn pop_front(&self) -> Result<Option<T>, PoisonError<MutexGuard<'_, VecDeque<T>>>>
    {
        Ok(self.0.lock()?.pop_front())
    }

    pub fn downgrade(&self) -> AtomicBufferWeak<T>
    {
        AtomicBufferWeak(Arc::downgrade(&self.0))
    }
}

