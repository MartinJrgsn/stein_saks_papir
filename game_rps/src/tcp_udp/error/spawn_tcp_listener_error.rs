#[derive(Debug)]
pub enum SpawnTcpListenerError
{
    LocalIpAdressError(local_ip_address::Error),
    BindError(std::io::Error),
    SpawnThreadError(std::io::Error)
}
impl From<local_ip_address::Error> for SpawnTcpListenerError
{
    fn from(error: local_ip_address::Error) -> Self
    {
        SpawnTcpListenerError::LocalIpAdressError(error)
    }
}