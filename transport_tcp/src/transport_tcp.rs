use std::net::TcpStream;

use atomic_buffer::AtomicBufferWeak;
use transport::transport::{TransportObj, SerdeTransport, StreamTransport};

use super::*;

#[derive(Clone, Copy)]
pub struct TransportTcp;

impl TransportObj for TransportTcp
{
    
}
impl<M> StreamTransport<M> for TransportTcp
{
    type StreamError = TcpStreamError;
    type StreamArgs = TcpStream;
    type SpawnStreamError = TcpSpawnStreamError;

    fn new_stream_args(target: Self::Target) -> Result<(Self::Id, Self::StreamArgs), Self::SpawnStreamError>
    {
        todo!()
    }
    fn stream_loop(
        transport: std::sync::Weak<std::sync::RwLock<Self>>,
        buffer: AtomicBufferWeak<M>,
        args: Self::StreamArgs
    )
        -> Self::StreamError
    {
        todo!()
    }
}