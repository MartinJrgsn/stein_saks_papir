#[derive(Debug)]
pub enum NewSessionTcpClientError
{
    ConnectError(std::io::Error),
    SpawnThreadError(std::io::Error)
}