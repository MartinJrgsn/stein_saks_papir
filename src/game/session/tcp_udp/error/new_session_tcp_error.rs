use super::*;

#[derive(Debug)]
pub enum NewSessionTcpError
{
    Host(NewSessionTcpHostError),
    Client(NewSessionTcpClientError)
}
impl From<NewSessionTcpHostError> for NewSessionTcpError
{
    fn from(error: NewSessionTcpHostError) -> Self
    {
        NewSessionTcpError::Host(error)
    }
}
impl From<NewSessionTcpClientError> for NewSessionTcpError
{
    fn from(error: NewSessionTcpClientError) -> Self
    {
        NewSessionTcpError::Client(error)
    }
}