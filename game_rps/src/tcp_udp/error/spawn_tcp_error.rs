use super::*;

#[derive(Debug)]
pub enum NewSessionTcpError
{
    Host(SpawnTcpListenerError),
    Client(SpawnTcpStreamError)
}
impl From<SpawnTcpListenerError> for NewSessionTcpError
{
    fn from(error: SpawnTcpListenerError) -> Self
    {
        NewSessionTcpError::Host(error)
    }
}
impl From<SpawnTcpStreamError> for NewSessionTcpError
{
    fn from(error: SpawnTcpStreamError) -> Self
    {
        NewSessionTcpError::Client(error)
    }
}