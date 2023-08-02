use transport::error::SpawnThreadError;

#[derive(Debug)]
pub enum SpawnTcpListenerError
{
    SpawnThreadError(SpawnThreadError),
    GetLocalIpAddressError(local_ip_address::Error),
    BindError(std::io::Error)
}

impl From<SpawnThreadError> for SpawnTcpListenerError
{
    fn from(error: SpawnThreadError) -> Self
    {
        Self::SpawnThreadError(error)
    }
}

impl From<local_ip_address::Error> for SpawnTcpListenerError
{
    fn from(error: local_ip_address::Error) -> Self
    {
        Self::GetLocalIpAddressError(error)
    }
}