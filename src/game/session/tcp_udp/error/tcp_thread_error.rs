use crate::game::DeserializeTcpError;

pub enum TcpThreadError
{
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