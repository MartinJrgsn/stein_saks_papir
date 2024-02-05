use atomic_buffer::error::BufferError;
use thiserror::Error;
use transport::error::JoinThreadError;

use crate::TcpConnectError;

#[derive(Error, Debug)]
pub enum TcpStreamError
{
    #[error("Buffer is null or poisoned.")]
    BufferError(BufferError),
    #[error("Unable to join TCP-stream thread.")]
    JoinError(JoinThreadError)
}

impl From<BufferError> for TcpStreamError
{
    fn from(error: BufferError) -> Self
    {
        Self::BufferError(error)
    }
}

impl From<JoinThreadError> for TcpStreamError
{
    fn from(error: JoinThreadError) -> Self
    {
        Self::JoinError(error)
    }
}