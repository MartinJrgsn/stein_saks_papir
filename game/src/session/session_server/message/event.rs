use std::vec;

use crate::{tcp_udp::{TryDeserializeTcp, DeserializeTcpError, SerializeTcp}, transport::Port};

use super::*;

pub enum ServerEvent
{
    OnKick(Port)
}
impl ServerEvent
{
    const ON_KICK: u8 = 0;
}
impl TryDeserializeTcp for ServerEvent
{
    fn try_deserialize_tcp(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::ON_KICK => Ok(Self::OnKick({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?
                    .split_array_ref().0
                    .clone();
                Port::from_le_bytes(bytes)
            })),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for ServerEvent
{
    fn serialize_tcp(&self) -> Vec<u8>
    {
        match self
        {
            Self::OnKick(port) => [
                vec![Self::ON_KICK],
                port.to_le_bytes().to_vec()
            ].concat()
        }
    }
}