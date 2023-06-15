#[derive(Debug)]
pub enum SpawnTcpStreamError
{
    ConnectError(std::io::Error),
    SpawnThreadError(std::io::Error)
}