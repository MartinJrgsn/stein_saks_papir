use super::*;

#[derive(Debug)]
pub enum ApplicationError
{
    NewSessionTcpError(NewSessionTcpError),
    RequestJoinError(RequestJoinError)
}
impl From<NewSessionTcpError> for ApplicationError
{
    fn from(error: NewSessionTcpError) -> Self
    {
        Self::NewSessionTcpError(error)
    }
}
impl From<RequestJoinError> for ApplicationError
{
    fn from(error: RequestJoinError) -> Self
    {
        Self::RequestJoinError(error)
    }
}