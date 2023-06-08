#[derive(Debug)]
pub enum NewSessionTcpHostError
{
    LocalIpAdressError(local_ip_address::Error),
    BindError(std::io::Error),
    SpawnThreadError(std::io::Error)
}
impl From<local_ip_address::Error> for NewSessionTcpHostError
{
    fn from(error: local_ip_address::Error) -> Self
    {
        NewSessionTcpHostError::LocalIpAdressError(error)
    }
}