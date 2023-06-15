use crate::tcp_udp::{NewSessionTcpError, SpawnTcpListenerError, SpawnTcpStreamError};

use super::*;

#[derive(Debug)]
pub enum ApplicationError
{
    NewSessionTcpError(NewSessionTcpError),
    RequestJoinError(RequestJoinError),
    ThreadError(Box<dyn std::any::Any + Send + 'static>),
    StdIoError(std::io::Error),
    GameRpsError(GameRpsError)
}
impl From<std::io::Error> for ApplicationError
{
    fn from(error: std::io::Error) -> Self
    {
        Self::StdIoError(error)
    }
}
impl From<NewSessionTcpError> for ApplicationError
{
    fn from(error: NewSessionTcpError) -> Self
    {
        Self::NewSessionTcpError(error)
    }
}
impl From<SpawnTcpStreamError> for ApplicationError
{
    fn from(error: SpawnTcpStreamError) -> Self
    {
        Self::NewSessionTcpError(error.into())
    }
}
impl From<SpawnTcpListenerError> for ApplicationError
{
    fn from(error: SpawnTcpListenerError) -> Self
    {
        Self::NewSessionTcpError(error.into())
    }
}
impl From<RequestJoinError> for ApplicationError
{
    fn from(error: RequestJoinError) -> Self
    {
        Self::RequestJoinError(error)
    }
}
impl From<GameRpsError> for ApplicationError
{
    fn from(error: GameRpsError) -> Self
    {
        Self::GameRpsError(error)
    }
}