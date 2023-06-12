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
impl From<RequestError> for RequestJoinError
{
    fn from(error: RequestError) -> Self
    {
        Self::RequestError(error)
    }
}
impl<T> From<std::sync::PoisonError<T>> for RequestJoinError
{
    fn from(error: std::sync::PoisonError<T>) -> Self
    {
        Self::RequestError(error.into())
    }
}
impl From<PoisonError> for RequestJoinError
{
    fn from(error: PoisonError) -> Self
    {
        Self::RequestError(error.into())
    }
}
impl From<std::time::SystemTimeError> for RequestJoinError
{
    fn from(error: std::time::SystemTimeError) -> Self
    {
        Self::RequestError(error.into())
    }
}
impl From<SendError> for RequestJoinError
{
    fn from(error: SendError) -> Self
    {
        Self::RequestError(error.into())
    }
}
impl From<DeserializeTcpError> for RequestJoinError
{
    fn from(error: DeserializeTcpError) -> Self
    {
        Self::RequestError(error.into())
    }
}