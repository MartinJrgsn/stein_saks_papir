use thiserror::Error;
use transport::error::SpawnThreadError;

#[derive(Error, Debug)]
pub enum SpawnTcpStreamError
{
    #[error("Unable to spawn thread for TCP-stream.")]
    SpawnThread(SpawnThreadError),
    #[error("Connection error.")]
    ConnectError(std::io::Error)
}

impl From<SpawnThreadError> for SpawnTcpStreamError
{
    fn from(error: SpawnThreadError) -> Self
    {
        Self::SpawnThread(error)
    }
}