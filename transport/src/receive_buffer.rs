use atomic_buffer::{error::BufferError, AtomicBufferWeak, AtomicBuffer};
use poison_error_obj::PoisonErrorUnguarded;

use crate::transport::{StreamTransport, Transport};

pub trait ReceiveBuffer<MessageType, TransportType>: Send + Sync + Clone
where
    TransportType: StreamTransport<MessageType>
{
    fn push_back(&self, message_result: Result<MessageType, TransportType::MessageError>) -> Result<(), BufferError>;
}

impl<M, T> ReceiveBuffer<M, T> for AtomicBufferWeak<Result<M, <T as Transport>::MessageError>>
where
    M: Send + Sync,
    T: StreamTransport<M>
{
    fn push_back(&self, message_result: Result<M, <T>::MessageError>) -> Result<(), BufferError>
    {
        self.push_back(message_result)
    }
}

impl<M, T> ReceiveBuffer<M, T> for AtomicBuffer<Result<M, <T as Transport>::MessageError>>
where
    M: Send + Sync,
    T: StreamTransport<M>
{
    fn push_back(&self, message_result: Result<M, <T>::MessageError>) -> Result<(), BufferError>
    {
        self.push_back(message_result).map_err(|_error| BufferError::PoisonError(PoisonErrorUnguarded))
    }
}

#[derive(Debug)]
pub struct ReceiveBufferShare<MessageType, TransportType>
where
    TransportType: StreamTransport<MessageType>
{
    id: TransportType::Target,
    buffer: AtomicBufferWeak<(TransportType::Target, Result<MessageType, TransportType::MessageError>)>
}

impl<M, T> Clone for ReceiveBufferShare<M, T>
where
    T: StreamTransport<M>
{
    fn clone(&self) -> Self
    {
        Self {
            id: self.id.clone(),
            buffer: self.buffer.clone()
        }
    }
}

impl<M, T> ReceiveBufferShare<M, T>
where
    T: StreamTransport<M>
{
    pub fn new(
        id: T::Target,
        buffer: AtomicBufferWeak<(T::Target, Result<M, T::MessageError>)>
    ) -> Self
    {
        Self {
            id,
            buffer
        }
    }
}

impl<M, T> ReceiveBuffer<M, T> for ReceiveBufferShare<M, T>
where
    M: Send + Sync,
    T: StreamTransport<M>
{
    fn push_back(&self, message_result: Result<M, T::MessageError>) -> Result<(), BufferError>
    {
        self.buffer.push_back((self.id, message_result))
    }
}