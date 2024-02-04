use thiserror::Error;
use transport::error::SpawnThreadError;

#[derive(Error, Debug)]
pub enum SpawnTcpListenerError
{
    #[error("Unable to spawn thread for TCP-listener.")]
    SpawnThreadError(SpawnThreadError),
    #[error("Unable to fetch local IP-address.")]
    GetLocalIpAddressError(local_ip_address::Error),
    #[error("Unable to bind TCP-listener to the given IP-address.")]
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