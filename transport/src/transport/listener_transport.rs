use std::sync::{Weak, RwLock};

use atomic_buffer::AtomicBufferWeak;

use crate::{error::SpawnThreadError, ParaStream, ReceiveBufferShare};

use super::StreamTransport;

pub trait ListenerTransport<MessageType>: StreamTransport<MessageType>
where
    MessageType: Send + Sync
{
    type ListenerError: Send;
    type ListenerArgs: Send + 'static;
    type SpawnListenerError: From<SpawnThreadError>;
    
    type ConnectError: Send;

    fn listener_loop(
        transport: &Weak<RwLock<Self>>,
        buffer_incoming: &AtomicBufferWeak<(Self::Target, Result<ParaStream<MessageType, Self, ReceiveBufferShare<MessageType, Self>>, Self::ConnectError>)>,
        buffer_receive: &AtomicBufferWeak<(Self::Target, Result<MessageType, Self::MessageError>)>,
        args: &mut Self::ListenerArgs
    ) -> Result<(), Self::ListenerError>;

    fn bind_listener(id: Self::Target) -> Result<(Self::Target, Self::ListenerArgs), Self::SpawnListenerError>;

    fn missing_connection_error(target: Self::Target) -> Self::ListenerError;
}