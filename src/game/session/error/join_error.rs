use super::*;

use thiserror::Error;

#[repr(u8)]
#[derive(Error, Debug)]
pub enum JoinError
{
    #[error("The session is full. Please wait until players have left or the game is finished.")]
    GameFull = Self::GAME_FULL,
    #[error("You have already joined this session.")]
    AlreadyJoined = Self::ALREADY_JOINED
}
impl JoinError
{
    const GAME_FULL: u8 = 0;
    const ALREADY_JOINED: u8 = 1;
}
impl TryDeserializeTcp for JoinError
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
        
        match header
        {
            Self::GAME_FULL => Ok(Self::GameFull),
            Self::ALREADY_JOINED => Ok(Self::AlreadyJoined),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for JoinError
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        match self
        {
            Self::GameFull => vec![Self::GAME_FULL],
            Self::AlreadyJoined => vec![Self::ALREADY_JOINED]
        }
    }
}