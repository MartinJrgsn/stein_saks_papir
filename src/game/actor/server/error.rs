use crate::game::{SessionJoinError, TryDeserializeTcp, SerializeTcp, DeserializeTcpError};

pub enum ActorServerHandleMessageError
{
    ThreadPosioned,
    JoinError(SessionJoinError)
}
impl ActorServerHandleMessageError
{
    const THREAD_POISONED_HEADER: u8 = 0;
    const JOIN_ERROR_HEADER: u8 = 1;
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
impl TryDeserializeTcp for ActorServerHandleMessageError
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::THREAD_POISONED_HEADER => Ok(Self::ThreadPosioned),
            Self::JOIN_ERROR_HEADER => Ok(Self::JoinError({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                SessionJoinError::try_from_tcp_message(bytes)?
            })),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for ActorServerHandleMessageError
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        match self
        {
            Self::ThreadPosioned => vec![Self::THREAD_POISONED_HEADER],
            Self::JoinError(error) => [vec![Self::JOIN_ERROR_HEADER], error.into_tcp_message()].concat()
        }
    }
}