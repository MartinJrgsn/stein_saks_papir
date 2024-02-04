use std::marker::PhantomData;

use atomic_buffer::{error::BufferError, AtomicBufferWeak, AtomicBuffer};
use poison_error_obj::PoisonErrorUnguarded;

use crate::transport::{StreamTransport, Transport};

pub trait ReceiveBuffer<RequestType, ResponseType, TransportType>: Send + Sync + Clone
where
    TransportType: StreamTransport<RequestType, ResponseType>
{
    fn push_back(&self, message_result: Result<ResponseType, TransportType::MessageError>) -> Result<(), BufferError>;
}

impl<MI, MO, T> ReceiveBuffer<MI, MO, T> for AtomicBufferWeak<Result<MO, <T as Transport>::MessageError>>
where
    MI: Send + Sync,
    MO: Send + Sync,
    T: StreamTransport<MI, MO>
{
    fn push_back(&self, message_result: Result<MO, <T>::MessageError>) -> Result<(), BufferError>
    {
        self.push_back(message_result)
    }
}

impl<MI, MO, T> ReceiveBuffer<MI, MO, T> for AtomicBuffer<Result<MO, <T as Transport>::MessageError>>
where
    MI: Send + Sync,
    MO: Send + Sync,
    T: StreamTransport<MI, MO>
{
    fn push_back(&self, message_result: Result<MO, <T>::MessageError>) -> Result<(), BufferError>
    {
        self.push_back(message_result).map_err(|_error| BufferError::PoisonError(PoisonErrorUnguarded))
    }
}

#[derive(Debug)]
pub struct ReceiveBufferShare<RequestType, ResponseType, TransportType>
where
    TransportType: StreamTransport<RequestType, ResponseType>
{
    id: TransportType::Target,
    buffer: AtomicBufferWeak<(TransportType::Target, Result<ResponseType, TransportType::MessageError>)>,
    phantom: PhantomData<RequestType>
}

impl<MI, MO, T> Clone for ReceiveBufferShare<MI, MO, T>
where
    T: StreamTransport<MI, MO>
{
    fn clone(&self) -> Self
    {
        Self {
            id: self.id.clone(),
            buffer: self.buffer.clone(),
            phantom: PhantomData::default()
        }
    }
}

impl<MI, MO, T> ReceiveBufferShare<MI, MO, T>
where
    T: StreamTransport<MI, MO>
{
    pub fn new(
        id: T::Target,
        buffer: AtomicBufferWeak<(T::Target, Result<MO, T::MessageError>)>
    ) -> Self
    {
        Self {
            id,
            buffer,
            phantom: PhantomData::default()
        }
    }
}

impl<MI, MO, T> ReceiveBuffer<MI, MO, T> for ReceiveBufferShare<MI, MO, T>
where
    MI: Send + Sync,
    MO: Send + Sync,
    T: StreamTransport<MI, MO>
{
    fn push_back(&self, message_result: Result<MO, T::MessageError>) -> Result<(), BufferError>
    {
        self.buffer.push_back((self.id, message_result))
    }
}