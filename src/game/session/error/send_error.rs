#[derive(Debug)]
pub enum SendError
{
    Deadlock,
    ThreadPoisoned,
    SystemTimeError(std::time::SystemTimeError)
}
impl<T> From<std::sync::PoisonError<T>> for SendError
{
    fn from(value: std::sync::PoisonError<T>) -> Self
    {
        Self::ThreadPoisoned
    }
}
impl From<std::time::SystemTimeError> for SendError
{
    fn from(error: std::time::SystemTimeError) -> Self
    {
        Self::SystemTimeError(error)
    }
}