use super::*;

#[derive(Debug)]
#[repr(u8)]
pub enum HandleClientMessageError
{
    ThreadPoisoned = Self::THREAD_POISONED,
    CannotRetrieveAddressFromStream = Self::CANNOT_RETRIEVE_ADDRESS_FROM_STREAM
}
impl HandleClientMessageError
{
    const THREAD_POISONED: u8 = 0;
    const CANNOT_RETRIEVE_ADDRESS_FROM_STREAM: u8 = 1;
}
impl<T> From<std::sync::PoisonError<T>> for HandleClientMessageError
{
    fn from(_: std::sync::PoisonError<T>) -> Self
    {
        HandleClientMessageError::ThreadPoisoned
    }
}
impl TryFrom<TcpThreadError> for HandleClientMessageError
{
    type Error = TcpThreadError;
    fn try_from(value: TcpThreadError) -> Result<Self, Self::Error>
    {
        match &value
        {
            TcpThreadError::ThreadPoisoned => Ok(Self::ThreadPoisoned),
            //TcpThreadError::CannotRetrieveAddressFromStream(_) => Ok(Self::CannotRetrieveAddressFromStream),
            _ => Err(value),
        }
    }
}
impl TryDeserializeTcp for HandleClientMessageError
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::THREAD_POISONED => Ok(Self::ThreadPoisoned),
            Self::CANNOT_RETRIEVE_ADDRESS_FROM_STREAM => Ok(Self::CannotRetrieveAddressFromStream),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for HandleClientMessageError
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        match self
        {
            Self::ThreadPoisoned => vec![Self::THREAD_POISONED],
            Self::CannotRetrieveAddressFromStream => vec![Self::CANNOT_RETRIEVE_ADDRESS_FROM_STREAM]
        }
    }
}