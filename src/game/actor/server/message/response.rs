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
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::ON_JOIN => Ok(Self::OnJoin({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                OnJoinEvent::try_from_tcp_message(bytes)?
            })),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for ServerResponse
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        match self
        {
            Self::OnJoin(event) => [
                vec![Self::ON_JOIN],
                event.into_tcp_message()
            ].concat(),
        }
    }
}