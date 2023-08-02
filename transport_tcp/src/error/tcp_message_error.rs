#[derive(Debug)]
pub enum TcpMessageError
{
    ReadError(TcpMessageReadError),
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

#[derive(Debug)]
pub enum TcpMessageReadError
{
    ReadFromStreamError(std::io::Error),
    DeserializeError(Box<bincode::ErrorKind>)
}

#[derive(Debug)]
pub enum TcpMessageWriteError
{
    WriteToStreamError(std::io::Error),
    SerializeError(Box<bincode::ErrorKind>)
}