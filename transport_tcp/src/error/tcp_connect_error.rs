use super::*;

#[derive(Debug)]
pub enum TcpConnectError
{
    SpawnTcpStreamError(SpawnTcpStreamError),
    LocalAddrError(std::io::Error)
}