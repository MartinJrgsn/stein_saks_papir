use std::{fmt::Display, time::SystemTimeError};
use thiserror::Error;

use super::*;

#[repr(u8)]
#[derive(Error, Debug)]
pub enum SessionJoinError
{
    #[error("The session is full. Please wait until players have left or the game is finished.")]
    GameFull = Self::GAME_FULL_HEADER,
    #[error("The thread is poisoned.")]
    ThreadPosioned = Self::THREAD_POISONED_HEADER,
    #[error("System time error.")]
    SystemTimeError(std::time::SystemTimeError) = Self::SYSTEM_TIME_ERROR_HEADER,
    #[error("Timeout.")]
    Timeout = Self::TIMEOUT_HEADER,
    #[error("You have already joined this session.")]
    AlreadyJoined = Self::ALREADY_JOINED_HEADER
}
impl SessionJoinError
{
    const GAME_FULL_HEADER: u8 = 0;
    const THREAD_POISONED_HEADER: u8 = 1;
    const SYSTEM_TIME_ERROR_HEADER: u8 = 2;
    const TIMEOUT_HEADER: u8 = 3;
    const ALREADY_JOINED_HEADER: u8 = 4;
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
impl TryDeserializeTcp for SessionJoinError
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
        
        match header
        {
            Self::GAME_FULL_HEADER => Ok(Self::GameFull),
            Self::THREAD_POISONED_HEADER => Ok(Self::ThreadPosioned),
            Self::SYSTEM_TIME_ERROR_HEADER => Ok(Self::SystemTimeError({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                std::time::SystemTimeError::try_from_tcp_message(bytes)?
            })),
            Self::TIMEOUT_HEADER => Ok(Self::Timeout),
            Self::ALREADY_JOINED_HEADER => Ok(Self::AlreadyJoined),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for SessionJoinError
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        match self
        {
            Self::GameFull => vec![Self::GAME_FULL_HEADER],
            Self::ThreadPosioned => vec![Self::THREAD_POISONED_HEADER],
            Self::SystemTimeError(error) => [
                vec![Self::SYSTEM_TIME_ERROR_HEADER],
                error.into_tcp_message()
            ].concat(),
            Self::Timeout => vec![Self::TIMEOUT_HEADER],
            Self::AlreadyJoined => vec![Self::ALREADY_JOINED_HEADER]
        }
    }
}