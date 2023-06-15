use crate::transport::SpawnThreadError;

use super::*;

pub enum TcpThreadError
{
    SpawnThreadError(SpawnThreadError),
    InvalidStream(std::io::Error),
    ReadStreamError(std::io::Error),
    WriteStreamError(std::io::Error),
    CannotRetrieveAddressFromStream(std::io::Error),
    ListenerStopped,
    ThreadPoisoned,
    DeserializeTcpError(DeserializeTcpError)
}
impl<T> From<std::sync::PoisonError<T>> for TcpThreadError
{
    fn from(value: std::sync::PoisonError<T>) -> Self
    {
        Self::ThreadPoisoned
    }
}
impl From<DeserializeTcpError> for TcpThreadError
{
    fn from(error: DeserializeTcpError) -> Self
    {
        TcpThreadError::DeserializeTcpError(error)
    }
}
impl From<SpawnThreadError> for TcpThreadError
{
    fn from(error: SpawnThreadError) -> Self
    {
        TcpThreadError::SpawnThreadError(error)
    }
}