use std::{error::Error, sync::{RwLock, Weak}};

use atomic_buffer::{error::BufferError, AtomicBufferWeak};

use crate::{error::SpawnThreadError, ParaStream, ReceiveBufferShare};

use super::StreamTransport;

pub trait ListenerTransport<RequestType, ResponseType>: StreamTransport<RequestType, ResponseType>
where
    RequestType: Send + Sync,
    ResponseType: Send + Sync
{
    type ListenerError: Send + Error + From<Self::ConnectError> + From<Self::StreamError> + From<BufferError>;
    type ListenerArgs: Send + 'static;
    type SpawnListenerError: From<SpawnThreadError> + Error;
    
    type ConnectError: Send + Error;

    fn listener_loop(
        transport: &Weak<RwLock<Self>>,
        buffer_incoming: &AtomicBufferWeak<(Self::Target, Result<ParaStream<RequestType, ResponseType, Self, ReceiveBufferShare<RequestType, ResponseType, Self>>, Self::ConnectError>)>,
        buffer_receive: &AtomicBufferWeak<(Self::Target, Result<ResponseType, Self::MessageError>)>,
        args: &mut Self::ListenerArgs
    ) -> Result<(), Self::ListenerError>;

    fn bind_listener(id: Self::Target) -> Result<(Self::Target, Self::ListenerArgs), Self::SpawnListenerError>;

    fn missing_connection_error(target: Self::Target) -> Self::ListenerError;
}