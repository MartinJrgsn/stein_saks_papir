pub mod response;

pub use response::*;

use super::*;

#[repr(u8)]
pub enum ServerMessage
{
    Response(ServerResponse) = Self::RESPONSE,
    Error(HandleClientMessageError) = Self::ERROR
}
impl ServerMessage
{
    const RESPONSE: u8 = 0;
    const ERROR: u8 = 255;
}
impl From<ServerResponse> for ServerMessage
{
    fn from(value: ServerResponse) -> Self
    {
        Self::Response(value)
    }
}
impl From<HandleClientMessageError> for ServerMessage
{
    fn from(error: HandleClientMessageError) -> Self
    {
        Self::Error(error)
    }
}
impl TryFrom<Result<ServerResponse, TcpThreadError>> for ServerMessage
{
    type Error = TcpThreadError;
    fn try_from(value: Result<ServerResponse, TcpThreadError>) -> Result<Self, TcpThreadError>
    {
        Ok(match value
        {
            Ok(response) => ServerMessage::Response(response),
            Err(error) => ServerMessage::Error(HandleClientMessageError::try_from(error)?)
        })
    }
}
impl TryDeserializeTcp for ServerMessage
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::RESPONSE => Ok(ServerMessage::Response({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                ServerResponse::try_from_tcp_message(bytes)?
            })),
            Self::ERROR => Ok(ServerMessage::Error({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                HandleClientMessageError::try_from_tcp_message(bytes)?
            })),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for ServerMessage
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        match self
        {
            Self::Response(response) => [
                vec![Self::RESPONSE],
                response.into_tcp_message()
            ].concat(),
            Self::Error(error) => [
                vec![Self::ERROR],
                error.into_tcp_message()
            ].concat()
        }
    }
}