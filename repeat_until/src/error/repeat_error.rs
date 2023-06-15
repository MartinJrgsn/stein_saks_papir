use std::time::Duration;

#[derive(Debug)]
pub enum RepeatError
{
    Timeout(Duration),
    SystemTimeError(std::time::SystemTimeError)
}
impl From<std::time::SystemTimeError> for RepeatError
{
    fn from(error: std::time::SystemTimeError) -> Self
    {
        Self::SystemTimeError(error)
    }
}