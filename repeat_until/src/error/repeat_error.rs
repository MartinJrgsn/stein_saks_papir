use std::{fmt::Display, time::Duration};

use thiserror::Error;

use crate::TimeoutError;

#[derive(Error, Debug, Clone)]
pub enum RepeatError
{
    Timeout(TimeoutError),
    SystemTimeError(std::time::SystemTimeError)
}
impl From<std::time::SystemTimeError> for RepeatError
{
    fn from(error: std::time::SystemTimeError) -> Self
    {
        Self::SystemTimeError(error)
    }
}
impl From<TimeoutError> for RepeatError
{
    fn from(error: TimeoutError) -> Self
    {
        Self::Timeout(error)
    }
}
impl Display for RepeatError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Timeout(error) => write!(f, "Repeat Error: {}", error),
            Self::SystemTimeError(error) => write!(f, "Repeat Error: {}", error)
        }
    }
}