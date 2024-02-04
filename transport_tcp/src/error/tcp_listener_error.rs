use atomic_buffer::error::BufferError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TcpListenerError<Target>
{
    #[error("Unable to accept incoming connection.")]
    AcceptConnectionError(std::io::Error),
    #[error("Buffer is null or poisoned.")]
    BufferError(BufferError),
    #[error("Connection is missing.")]
    MissingConnectionError(Target)
}

impl<Target> From<BufferError> for TcpListenerError<Target>
{
    fn from(error: BufferError) -> Self
    {
        Self::BufferError(error)
    }
}