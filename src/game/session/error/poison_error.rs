pub struct PoisonError;

impl<T> From<std::sync::PoisonError<T>> for PoisonError
{
    fn from(value: std::sync::PoisonError<T>) -> Self
    {
        Self
    }
}