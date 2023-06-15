use super::*;

pub enum TryRepeatError<FnError>
{
    RepeatError(RepeatError),
    FnError(FnError)
}
impl<FnError> From<std::time::SystemTimeError> for TryRepeatError<FnError>
{
    fn from(error: std::time::SystemTimeError) -> Self
    {
        Self::RepeatError(error.into())
    }
}
impl<FnError> From<RepeatError> for TryRepeatError<FnError>
{
    fn from(error: RepeatError) -> Self
    {
        Self::RepeatError(error)
    }
}