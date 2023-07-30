use std::{sync::{Weak, RwLock, PoisonError, MutexGuard}, thread::JoinHandle, collections::VecDeque};

use atomic_buffer::{AtomicBuffer, AtomicBufferWeak};

use super::*;

/// A stream running in a paralell process (another thread)
pub struct ParaStream<MessageType, TransportType>
where
    TransportType: StreamTransport<MessageType>
{
    target: TransportType::Target,
    id: TransportType::Id,
    transport: Weak<RwLock<TransportType>>,
    thread: JoinHandle<TransportType::StreamError>,
    buffer: AtomicBuffer<MessageType>
}
impl<M, T> ParaStream<M, T>
where
    T: StreamTransport<M> + 'static,
    M: Send + Sync + 'static
{
    pub fn new(name: &str, target: T::Target, transport: Weak<RwLock<T>>)
        -> Result<Self, T::SpawnStreamError>
    {
        let buffer = AtomicBuffer::new();
        let (id, thread) = Self::new_thread(
            name,
            target,
            transport.clone(),
            buffer.downgrade()
        )?;
        Ok(Self {
            target,
            id,
            transport,
            thread,
            buffer
        })
    }
    fn new_thread(
        name: &str,
        target: T::Target,
        transport: Weak<RwLock<T>>,
        buffer: AtomicBufferWeak<M>
    )
        -> Result<(T::Id, JoinHandle<T::StreamError>), T::SpawnStreamError>
    {
        let (id, args) = T::new_stream_args(target)?;
        let mut builder = std::thread::Builder::new()
            .name(format!("{} ({} Stream to {} on {})", name, T::NAME, target, id));
        if let Some(stack_size) = T::LISTENER_STACK_SIZE
        {
            builder = builder.stack_size(stack_size)
        }
        Ok((id, builder
            .spawn(move || T::stream_loop(
                transport,
                buffer,
                args
            ))
            .map_err(|error| SpawnThreadError(error))?))
    }

    pub fn get_target(&self) -> T::Target
    {
        self.target
    }
    pub fn get_id(&self) -> T::Id
    {
        self.id
    }
    pub fn get_transport(&self) -> &Weak<RwLock<T>>
    {
        &self.transport
    }

    pub fn check_thread(self) -> Result<Self, T::StreamError>
    where
        T::StreamError: From<JoinError>
    {
        if self.thread.is_finished()
        {
            return Err(self.thread
                .join()
                .map_err(|error| JoinError(error))?
                .into()
            )
        }
        Ok(self)
    }

    pub fn send<'a>(&'a self, message: M) -> Result<(), T::StreamError>
    where
        T::StreamError: From<PoisonError<MutexGuard<'a, VecDeque<M>>>>
    {
        Ok(self.buffer.push_back(message)?)
    }
}