use super::*;

#[derive(Debug)]
pub enum RequestError
{
    Timeout,
    Stolen,
    SendError(SendError),
    ReceiveError(ReceiveError)
}
impl From<SendError> for RequestError
{
    fn from(error: SendError) -> Self
    {
        Self::SendError(error)
    }
}
impl<T> From<T> for RequestError
where
    T: Into<ReceiveError> + ?Sized
{
    fn from(error: T) -> Self
    {
        Self::ReceiveError(error.into())
    }
}