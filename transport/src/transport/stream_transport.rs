use std::sync::{Weak, RwLock};

use atomic_buffer::AtomicBufferWeak;

use crate::error::SpawnThreadError;

use super::Transport;

pub trait StreamTransport<MessageType>: Transport
{
    type StreamError: Send;
    type StreamArgs: Send + 'static;
    type SpawnStreamError: From<SpawnThreadError>;
    
    fn stream_loop(
        transport: Weak<RwLock<Self>>,
        buffer: AtomicBufferWeak<MessageType>,
        args: Self::StreamArgs
    )
        -> Self::StreamError;
    
    fn new_stream_args(target: Self::Target) -> Result<(Self::Id, Self::StreamArgs), Self::SpawnStreamError>;
}