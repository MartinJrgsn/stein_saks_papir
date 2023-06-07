use std::{fmt::Display, time::SystemTimeError};
use thiserror::Error;

use super::*;

#[derive(Error, Debug)]
pub enum SessionJoinError
{
    #[error("The session is full. Please wait until players have left or the game is finished.")]
    GameFull,
    #[error("The thread is poisoned.")]
    ThreadPosioned,
    #[error("System time error.")]
    SystemTimeError(std::time::SystemTimeError),
    #[error("Timeout.")]
    Timeout,
    #[error("You have already joined this session.")]
    AlreadyJoined
}
impl<T> From<std::sync::PoisonError<T>> for SessionJoinError
{
    fn from(error: std::sync::PoisonError<T>) -> Self
    {
        SessionJoinError::ThreadPosioned
    }
}
impl From<SystemTimeError> for SessionJoinError
{
    fn from(error: SystemTimeError) -> Self
    {
        SessionJoinError::SystemTimeError(error)
    }
}