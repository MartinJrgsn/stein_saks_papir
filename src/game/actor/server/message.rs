use crate::game::{TryDeserializeTcp, SerializeTcp, DeserializeTcpError};

use super::*;

pub enum ServerMessage
{
    JoinResponse(Port),
    Error(ActorServerHandleMessageError)
}
impl ServerMessage
{
    const JOIN_RESPONSE_HEADER: u8 = 0;
    const ERROR_HEADER: u8 = 255;
}
impl TryDeserializeTcp for ServerMessage
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::JOIN_RESPONSE_HEADER => Ok(ServerMessage::JoinResponse({
                // can be shortened to a one-liner in rust nightly with #![feature(split_array)]
                /*Port::from_be_bytes(
                    *bytes.get(1..)
                        .ok_or(ServerMessageParseError::InsufficientBufferLength(bytes.len()))?
                        .split_array_ref().0
                )*/

                const PORT_SIZE: usize = Port::BITS as usize/8;
                let mut iter = bytes.get(1..(1 + PORT_SIZE))
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?
                    .into_iter();
                let mut bytes = [0; PORT_SIZE];
                bytes.fill_with(|| *iter.next().unwrap());
                Port::from_le_bytes(bytes)
            })),
            Self::ERROR_HEADER => Ok(ServerMessage::Error({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                ActorServerHandleMessageError::try_from_tcp_message(bytes)?
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
            Self::JoinResponse(port) => [
                vec![Self::JOIN_RESPONSE_HEADER],
                port.to_le_bytes().to_vec()
            ].concat(),
            Self::Error(error) => [
                vec![Self::ERROR_HEADER],
                error.into_tcp_message()
            ].concat()
        }
    }
}