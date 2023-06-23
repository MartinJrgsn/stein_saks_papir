use std::sync::{Weak, RwLock};

use atomic_buffer::AtomicBufferWeak;

use crate::error::SpawnThreadError;

use super::Transport;

pub trait ListenerTransport<MessageType>: Transport
{
    type ListenerError: Send;
    type ListenerArgs: Send + 'static;
    type SpawnListenerError: From<SpawnThreadError>;

    fn listener_loop(
        transport: Weak<RwLock<Self>>,
        buffer: AtomicBufferWeak<(Self::Id, Result<MessageType, Self::DeserializeError>)>,
        args: Self::ListenerArgs
    )
        -> Self::ListenerError;

    fn new_listener_args(id: Self::Id) -> Result<(Self::Target, Self::ListenerArgs), Self::SpawnListenerError>;
}