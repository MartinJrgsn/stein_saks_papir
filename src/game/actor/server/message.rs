pub mod error;

pub use error::*;

use super::*;

pub enum ServerMessage
{
    JoinResponse(Port)
}
impl TryFrom<Vec<u8>> for ServerMessage
{
    type Error = ServerMessageParseError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error>
    {
        let header = *value.get(0)
            .ok_or(ServerMessageParseError::InsufficientBufferLength(value.len()))?;

        match header as usize
        {
            0 => Ok(ServerMessage::JoinResponse({
                // can be shortened to a one-liner in rust nightly with #![feature(split_array)]
                /*Port::from_be_bytes(
                    *value.get(1..)
                        .ok_or(ServerMessageParseError::InsufficientBufferLength(value.len()))?
                        .split_array_ref().0
                )*/

                const PORT_SIZE: usize = Port::BITS as usize/8;
                let mut bytes = [0; PORT_SIZE];
                let mut iter = value.get(1..(1 + PORT_SIZE))
                    .ok_or(ServerMessageParseError::InsufficientBufferLength(value.len()))?
                    .into_iter();
                bytes.fill_with(|| *iter.next().unwrap());
                Port::from_le_bytes(bytes)
            })),
            _ => Err(ServerMessageParseError::UnrecognizedHeader(header))
        }
    }
}