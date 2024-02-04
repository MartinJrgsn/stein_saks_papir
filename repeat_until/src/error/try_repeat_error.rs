use thiserror::Error;

use super::*;

#[derive(Error, Debug, Clone)]
pub enum TryRepeatError<FnError>
{
    RepeatError(RepeatError),
    FnError(FnError)
}
impl<FnError> TryRepeatError<FnError>
{
    pub fn flat_map(self, map: impl Fn(RepeatError) -> FnError) -> FnError
    {
        match self
        {
            Self::RepeatError(error) => map(error),
            Self::FnError(error) => error
        }
    }
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