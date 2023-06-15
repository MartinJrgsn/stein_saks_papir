use crate::tcp_udp::{DeserializeTcpError, TryDeserializeTcp, SerializeTcp};

use super::*;

#[repr(u8)]
pub enum ServerResponse
{
    OnJoin(OnJoinEvent) = Self::ON_JOIN,
}
impl ServerResponse
{
    const ON_JOIN: u8 = 0;
}
impl TryDeserializeTcp for ServerResponse
{
    fn try_deserialize_tcp(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::ON_JOIN => Ok(Self::OnJoin({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                OnJoinEvent::try_deserialize_tcp(bytes)?
            })),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for ServerResponse
{
    fn serialize_tcp(&self) -> Vec<u8>
    {
        match self
        {
            Self::OnJoin(event) => [
                vec![Self::ON_JOIN],
                event.serialize_tcp()
            ].concat(),
        }
    }
}