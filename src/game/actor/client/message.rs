pub mod error;

pub use error::*;

use super::*;

pub enum ClientMessage
{
    Select(Choice),
    Name(String)
}
impl TryFrom<Vec<u8>> for ClientMessage
{
    type Error = ClientMessageParseError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error>
    {
        let header = *value.get(0)
            .ok_or(ClientMessageParseError::InsufficientBufferLength(value.len()))?;

        if header < Choice::LENGTH as u8
        {
            return Ok(ClientMessage::Select(Choice::try_from(header).unwrap()))
        }
        match header as usize
        {
            Choice::LENGTH => Ok(ClientMessage::Name(
                String::from_utf8(value.get(1..)
                    .ok_or(ClientMessageParseError::InsufficientBufferLength(value.len()))?
                    .to_vec()
                )?
            )),
            _ => Err(ClientMessageParseError::UnrecognizedHeader(header))
        }
    }
}