use std::{sync::{Weak, Mutex, Arc}, collections::VecDeque};

use crate::error::BufferError;

pub struct AtomicBufferWeak<T>(pub(super) Weak<Mutex<VecDeque<T>>>);

impl<T> AtomicBufferWeak<T>
{
    fn arc(&self) -> Result<Arc<Mutex<VecDeque<T>>>, BufferError>
    {
        Ok(self.0.upgrade().ok_or_else(|| BufferError::NullPointerError)?)
    }

    pub fn push_back(&self, value: T) -> Result<(), BufferError>
    {
        Ok(self.arc()?.lock()?.push_back(value))
    }

    pub fn pop_front(&self) -> Result<Option<T>, BufferError>
    {
        Ok(self.arc()?.lock()?.pop_front())
    }
}

