use atomic_buffer::error::BufferError;
use thiserror::Error;
use transport::error::JoinThreadError;

use crate::{TcpConnectError, TcpStreamError};

#[derive(Error, Debug)]
pub enum TcpListenerError<Target>
{
    #[error("Unable to accept incoming connection.")]
    AcceptConnectionError(std::io::Error),
    #[error("Buffer is null or poisoned.")]
    BufferError(BufferError),
    #[error("Connection is missing.")]
    MissingConnectionError(Target),
    #[error("TCP connection error.")]
    TcpConnectError(TcpConnectError),
    #[error("Unable to join TCP-listener thread.")]
    JoinError(JoinThreadError),
    #[error("TCP-stream error.")]
    TcpStreamError(TcpStreamError)
}

impl<Target> From<BufferError> for TcpListenerError<Target>
{
    fn from(error: BufferError) -> Self
    {
        Self::BufferError(error)
    }
}
impl<Target> From<TcpConnectError> for TcpListenerError<Target>
{
    fn from(error: TcpConnectError) -> Self
    {
        Self::TcpConnectError(error)
    }
}
impl<Target> From<JoinThreadError> for TcpListenerError<Target>
{
    fn from(error: JoinThreadError) -> Self
    {
        Self::JoinError(error)
    }
}
impl<Target> From<TcpStreamError> for TcpListenerError<Target>
{
    fn from(error: TcpStreamError) -> Self
    {
        Self::TcpStreamError(error)
    }
}