use crate::repeat_until::RepeatError;

use super::*;

#[derive(Debug)]
pub enum SendError
{
    Deadlock(RepeatError),
    ThreadPoisoned(PoisonError),
    SystemTimeError(std::time::SystemTimeError)
}
impl<T> From<T> for SendError
where T: Into<PoisonError> + ?Sized
{
    fn from(value: T) -> Self
    {
        Self::ThreadPoisoned(value.into())
    }
}
impl From<std::time::SystemTimeError> for SendError
{
    fn from(error: std::time::SystemTimeError) -> Self
    {
        Self::SystemTimeError(error)
    }
}