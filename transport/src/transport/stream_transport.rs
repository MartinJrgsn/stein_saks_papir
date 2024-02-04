use std::{error::Error, sync::{RwLock, Weak}};

use atomic_buffer::{error::BufferError, AtomicBufferWeak};

use crate::{error::SpawnThreadError, ReceiveBuffer};

use super::Transport;

pub trait StreamTransport<RequestType, ResponseType>: Transport
{
    type StreamError: Send + Error + From<BufferError>;
    type StreamArgs: Send + 'static;
    type SpawnStreamError: From<SpawnThreadError> + Error;
    
    fn stream_loop<B>(
        transport: &Weak<RwLock<Self>>,
        buffer_send: &AtomicBufferWeak<RequestType>,
        buffer_receive: &B,
        args: &mut Self::StreamArgs
    )
        -> Result<(), Self::StreamError>
    where
        B: ReceiveBuffer<RequestType, ResponseType, Self>;
    
    fn connect_stream(target: Self::Target) -> Result<(Self::Target, Self::StreamArgs), Self::SpawnStreamError>;
}