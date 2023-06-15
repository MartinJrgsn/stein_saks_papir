use super::*;

#[derive(Debug)]
pub enum RequestJoinError
{
    JoinError(JoinError),
    RequestError(RequestError),
    ServerErrorMessage(HandleClientMessageError),
    PromtError(PromtError)
}
impl From<PromtError> for RequestJoinError
{
    fn from(error: PromtError) -> Self
    {
        Self::PromtError(error)
    }
}
impl From<HandleClientMessageError> for RequestJoinError
{
    fn from(value: HandleClientMessageError) -> Self
    {
        Self::ServerErrorMessage(value)
    }
}
impl From<JoinError> for RequestJoinError
{
    fn from(error: JoinError) -> Self
    {
        Self::JoinError(error)
    }
}
impl<T> From<T> for RequestJoinError
where T: Into<RequestError> + ?Sized
{
    fn from(error: T) -> Self
    {
        Self::RequestError(error.into())
    }
}