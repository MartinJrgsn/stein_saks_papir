pub mod response;
pub mod event;

pub use response::*;
pub use event::*;

use crate::tcp_udp::{TcpThreadError, DeserializeTcpError, TryDeserializeTcp, SerializeTcp};

use super::*;

#[repr(u8)]
pub enum ServerMessage
{
    Event(ServerEvent) = Self::EVENT,
    Response(ServerResponse) = Self::RESPONSE,
    Error(HandleClientMessageError) = Self::ERROR
}
impl ServerMessage
{
    const EVENT: u8 = 0;
    const RESPONSE: u8 = 1;
    const ERROR: u8 = 255;
}
impl From<ServerEvent> for ServerMessage
{
    fn from(value: ServerEvent) -> Self
    {
        Self::Event(value)
    }
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
    fn try_deserialize_tcp(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::EVENT => Ok(ServerMessage::Event({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                ServerEvent::try_deserialize_tcp(bytes)?
            })),
            Self::RESPONSE => Ok(ServerMessage::Response({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                ServerResponse::try_deserialize_tcp(bytes)?
            })),
            Self::ERROR => Ok(ServerMessage::Error({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                HandleClientMessageError::try_deserialize_tcp(bytes)?
            })),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for ServerMessage
{
    fn serialize_tcp(&self) -> Vec<u8>
    {
        match self
        {
            Self::Event(event) => [
                vec![Self::EVENT],
                event.serialize_tcp()
            ].concat(),
            Self::Response(response) => [
                vec![Self::RESPONSE],
                response.serialize_tcp()
            ].concat(),
            Self::Error(error) => [
                vec![Self::ERROR],
                error.serialize_tcp()
            ].concat()
        }
    }
}