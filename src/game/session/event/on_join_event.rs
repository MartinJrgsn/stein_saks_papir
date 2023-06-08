use super::*;

pub enum OnJoinEvent
{
    Success(Port),
    Failure(JoinError)
}
impl OnJoinEvent
{
    const SUCCESS: u8 = 0;
    const FAILURE: u8 = 1;
}
impl Into<Result<Port, JoinError>> for OnJoinEvent
{
    fn into(self) -> Result<Port, JoinError>
    {
        match self
        {
            Self::Success(port) => Ok(port),
            Self::Failure(error) => Err(error)
        }
    }
}
impl Into<Result<Port, RequestJoinError>> for OnJoinEvent
{
    fn into(self) -> Result<Port, RequestJoinError>
    {
        match self
        {
            Self::Success(port) => Ok(port),
            Self::Failure(error) => Err(error.into())
        }
    }
}
impl TryDeserializeTcp for OnJoinEvent
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        let header = *bytes.get(0)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;

        match header
        {
            Self::SUCCESS => Ok(Self::Success({
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
            Self::FAILURE => Ok(Self::Failure({
                let bytes = bytes.get(1..)
                    .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?;
                JoinError::try_from_tcp_message(bytes)?
            })),
            _ => Err(DeserializeTcpError::UnrecognizedHeader(header))
        }
    }
}
impl SerializeTcp for OnJoinEvent
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        match self
        {
            Self::Success(port) => [
                vec![Self::SUCCESS],
                port.to_le_bytes().to_vec()
            ].concat(),
            Self::Failure(error) => [
                vec![Self::FAILURE],
                error.into_tcp_message()
            ].concat()
        }
    }
}