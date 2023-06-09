use super::*;

#[derive(Debug)]
pub enum RequestError
{
    Timeout,
    Stolen,
    SendError(SendError),
    ResponseError(DeserializeTcpError)
}
impl<T> From<std::sync::PoisonError<T>> for RequestError
{
    fn from(error: std::sync::PoisonError<T>) -> Self
    {
        Self::SendError(error.into())
    }
}
impl From<PoisonError> for RequestError
{
    fn from(error: PoisonError) -> Self
    {
        Self::SendError(error.into())
    }
}
impl From<std::time::SystemTimeError> for RequestError
{
    fn from(error: std::time::SystemTimeError) -> Self
    {
        Self::SendError(error.into())
    }
}
impl From<SendError> for RequestError
{
    fn from(error: SendError) -> Self
    {
        Self::SendError(error)
    }
}
impl From<DeserializeTcpError> for RequestError
{
    fn from(error: DeserializeTcpError) -> Self
    {
        Self::ResponseError(error)
    }
}