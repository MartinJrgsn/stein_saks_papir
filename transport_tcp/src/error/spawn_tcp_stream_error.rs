use transport::error::SpawnThreadError;

#[derive(Debug)]
pub enum SpawnTcpStreamError
{
    SpawnThread(SpawnThreadError),
    ConnectError(std::io::Error)
}

impl From<SpawnThreadError> for SpawnTcpStreamError
{
    fn from(error: SpawnThreadError) -> Self
    {
        Self::SpawnThread(error)
    }
}