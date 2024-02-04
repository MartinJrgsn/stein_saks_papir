use thiserror::Error;

#[derive(Error, Debug)]
pub enum TcpMessageError
{
    #[error("Unable to read message.")]
    ReadError(TcpMessageReadError),
    #[error("Unable to write message.")]
    WriteError(TcpMessageWriteError)
}

impl From<TcpMessageReadError> for TcpMessageError
{
    fn from(error: TcpMessageReadError) -> Self
    {
        Self::ReadError(error)
    }
}
impl From<TcpMessageWriteError> for TcpMessageError
{
    fn from(error: TcpMessageWriteError) -> Self
    {
        Self::WriteError(error)
    }
}

#[derive(Error, Debug)]
pub enum TcpMessageReadError
{
    #[error("Unable to read from stream.")]
    ReadFromStreamError(std::io::Error),
    #[error("Deserialization error.")]
    DeserializeError(Box<bincode::ErrorKind>)
}

#[derive(Error, Debug)]
pub enum TcpMessageWriteError
{
    #[error("Unable to write to stream.")]
    WriteToStreamError(std::io::Error),
    #[error("Serialization error.")]
    SerializeError(Box<bincode::ErrorKind>)
}