use game::error::{PromptError, SessionError};
use transport::error::{JoinThreadError, SpawnThreadError};
use transport_tcp::error::{SpawnTcpListenerError, SpawnTcpStreamError};

use self::message::{RpsClientMessageData, RpsServerMessageData};

use super::*;

#[derive(Debug)]
pub enum ApplicationError
{
    SpawnTcpListenerError(SpawnTcpListenerError),
    SpawnTcpStreamError(SpawnTcpStreamError),
    SpawnThreadError(SpawnThreadError),
    JoinThreadError(JoinThreadError),
    SessionError(SessionError<RpsClientMessageData, RpsServerMessageData, SocketAddr, RpsEndState, TransportTcp>),
    PromptError(PromptError),
    GameRpsError(GameRpsError)
}
impl From<SessionError<RpsClientMessageData, RpsServerMessageData, SocketAddr, RpsEndState, TransportTcp>> for ApplicationError
{
    fn from(error: SessionError<RpsClientMessageData, RpsServerMessageData, SocketAddr, RpsEndState, TransportTcp>) -> Self
    {
        Self::SessionError(error)
    }
}
impl From<SpawnThreadError> for ApplicationError
{
    fn from(error: SpawnThreadError) -> Self
    {
        Self::SpawnThreadError(error)
    }
}
impl From<JoinThreadError> for ApplicationError
{
    fn from(error: JoinThreadError) -> Self
    {
        Self::JoinThreadError(error)
    }
}
impl From<PromptError> for ApplicationError
{
    fn from(error: PromptError) -> Self
    {
        Self::PromptError(error)
    }
}
impl From<SpawnTcpListenerError> for ApplicationError
{
    fn from(error: SpawnTcpListenerError) -> Self
    {
        Self::SpawnTcpListenerError(error)
    }
}
impl From<SpawnTcpStreamError> for ApplicationError
{
    fn from(error: SpawnTcpStreamError) -> Self
    {
        Self::SpawnTcpStreamError(error)
    }
}
impl From<GameRpsError> for ApplicationError
{
    fn from(error: GameRpsError) -> Self
    {
        Self::GameRpsError(error)
    }
}