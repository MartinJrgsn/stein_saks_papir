use crate::game::{TryDeserializeTcp, DeserializeTcpError, SerializeTcp};

use super::*;

#[repr(u8)]
pub enum ClientMessage
{
    Select(Choice) = Self::SELECT,
    Name(String) = Self::NAME
}
impl ClientMessage
{
    const SELECT: u8 = 0;
    const NAME: u8 = 1;
}
impl TryDeserializeTcp for ClientMessage
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::SELECT => Ok(Self::Select(
                Choice::try_from(
                    *bytes.get(0)
                        .ok_or_else(|| DeserializeTcpError::InsufficientBufferLength(bytes.len()))?
                ).map_err(|_| DeserializeTcpError::ChoiceParseError)?
            )),
            Self::NAME => Ok(Self::Name(
                String::from_utf8(bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?
                    .to_vec()
                )?
            )),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for ClientMessage
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        match self
        {
            Self::Select(choice) => [vec![Self::SELECT], choice.into_tcp_message()].concat(),
            Self::Name(name) => [vec![Self::NAME], name.clone().into_bytes()].concat()
        }
    }
}