use super::*;

#[derive(Debug)]
pub enum SessionTcpNewError
{
    Host(SessionTcpNewHostError),
    Client(SessionTcpNewClientError)
}
impl From<SessionTcpNewHostError> for SessionTcpNewError
{
    fn from(error: SessionTcpNewHostError) -> Self
    {
        SessionTcpNewError::Host(error)
    }
}
impl From<SessionTcpNewClientError> for SessionTcpNewError
{
    fn from(error: SessionTcpNewClientError) -> Self
    {
        SessionTcpNewError::Client(error)
    }
}

#[derive(Debug)]
pub enum SessionTcpNewHostError
{
    LocalIpAdressError(local_ip_address::Error),
    BindError(std::io::Error),
    SpawnThreadError(std::io::Error)
}
impl From<local_ip_address::Error> for SessionTcpNewHostError
{
    fn from(error: local_ip_address::Error) -> Self
    {
        SessionTcpNewHostError::LocalIpAdressError(error)
    }
}

#[derive(Debug)]
pub enum SessionTcpNewClientError
{
    
}