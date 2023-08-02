use atomic_buffer::error::BufferError;

#[derive(Debug)]
pub enum TcpListenerError<Target>
{
    AcceptConnectionError(std::io::Error),
    BufferError(BufferError),
    MissingConnectionError(Target)
}

impl<Target> From<BufferError> for TcpListenerError<Target>
{
    fn from(error: BufferError) -> Self
    {
        Self::BufferError(error)
    }
}