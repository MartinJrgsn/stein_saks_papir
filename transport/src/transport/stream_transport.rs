use std::sync::{Weak, RwLock};

use atomic_buffer::AtomicBufferWeak;

use crate::{error::SpawnThreadError, ReceiveBuffer};

use super::Transport;

pub trait StreamTransport<MessageType>: Transport
{
    type StreamError: Send;
    type StreamArgs: Send + 'static;
    type SpawnStreamError: From<SpawnThreadError>;
    
    fn stream_loop<B>(
        transport: &Weak<RwLock<Self>>,
        buffer_send: &AtomicBufferWeak<MessageType>,
        buffer_receive: &B,
        args: &mut Self::StreamArgs
    )
        -> Result<(), Self::StreamError>
    where
        B: ReceiveBuffer<MessageType, Self>;
    
    fn connect_stream(target: Self::Target) -> Result<(Self::Target, Self::StreamArgs), Self::SpawnStreamError>;
}