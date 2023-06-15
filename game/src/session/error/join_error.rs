use crate::tcp_udp::{TryDeserializeTcp, DeserializeTcpError, SerializeTcp};

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
impl<PlayerType> TryFrom<&AddSomePlayerError<PlayerType>> for JoinError
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    type Error = ();
    fn try_from(error: &AddSomePlayerError<PlayerType>) -> Result<Self, ()>
    {
        if let AddSomePlayerError::AddPlayerError(error) = error
        {
            return Ok(Self::from(error))
        }
        Err(())
    }
}
impl<PlayerType> From<&AddPlayerError<PlayerType>> for JoinError
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    fn from(error: &AddPlayerError<PlayerType>) -> Self
    {
        match error
        {
            AddPlayerError::AlreadyJoined(_) => JoinError::AlreadyJoined,
            AddPlayerError::GameFull(_) => JoinError::GameFull
        }
    }
}
impl TryDeserializeTcp for JoinError
{
    fn try_deserialize_tcp(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
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
    fn serialize_tcp(&self) -> Vec<u8>
    {
        match self
        {
            Self::GameFull => vec![Self::GAME_FULL],
            Self::AlreadyJoined => vec![Self::ALREADY_JOINED]
        }
    }
}