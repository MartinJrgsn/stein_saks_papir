use crate::game::SessionJoinError;

pub enum ActorServerHandleMessageError
{
    ThreadPosioned,
    JoinError(SessionJoinError)
}
impl<T> From<std::sync::PoisonError<T>> for ActorServerHandleMessageError
{
    fn from(error: std::sync::PoisonError<T>) -> Self
    {
        ActorServerHandleMessageError::ThreadPosioned
    }
}
impl From<SessionJoinError> for ActorServerHandleMessageError
{
    fn from(error: SessionJoinError) -> Self
    {
        ActorServerHandleMessageError::JoinError(error)
    }
}