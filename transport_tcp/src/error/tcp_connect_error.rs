use thiserror::Error;

use super::*;

#[derive(Error, Debug)]
pub enum TcpConnectError
{
    #[error("Unable to spawn TCP-stream.")]
    SpawnTcpStreamError(SpawnTcpStreamError),
    #[error("Unable to fetch the local address of the TCP-listener.")]
    LocalAddrError(std::io::Error)
}