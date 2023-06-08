use crate::game::{TryDeserializeTcp, DeserializeTcpError};

use super::*;

pub enum ClientMessage
{
    Select(Choice),
    Name(String)
}
impl TryDeserializeTcp for ClientMessage
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        if header < Choice::LENGTH as u8
        {
            return Ok(ClientMessage::Select(Choice::try_from(header).unwrap()))
        }
        match header as usize
        {
            Choice::LENGTH => Ok(ClientMessage::Name(
                String::from_utf8(bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?
                    .to_vec()
                )?
            )),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}