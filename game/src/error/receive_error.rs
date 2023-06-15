use crate::tcp_udp::DeserializeTcpError;

use super::*;

#[derive(Debug)]
pub enum ReceiveError
{
    ThreadPoisoned(PoisonError),
    DeserializeTcpError(DeserializeTcpError)
}
impl From<DeserializeTcpError> for ReceiveError
{
    fn from(error: DeserializeTcpError) -> Self
    {
        Self::DeserializeTcpError(error)
    }
}
impl<T> From<T> for ReceiveError
where
    T: Into<PoisonError>
{
    fn from(value: T) -> Self
    {
        Self::ThreadPoisoned(PoisonError)
    }
}