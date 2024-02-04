use atomic_buffer::error::BufferError;
use thiserror::Error;
use transport::error::JoinError;

use crate::TcpConnectError;

#[derive(Error, Debug)]
pub enum TcpStreamError
{
    #[error("Buffer is null or poisoned.")]
    BufferError(BufferError),
    #[error("Unable to join TCP-stream thread.")]
    JoinError(JoinError)
}

impl From<BufferError> for TcpStreamError
{
    fn from(error: BufferError) -> Self
    {
        Self::BufferError(error)
    }
}

impl From<JoinError> for TcpStreamError
{
    fn from(error: JoinError) -> Self
    {
        Self::JoinError(error)
    }
}