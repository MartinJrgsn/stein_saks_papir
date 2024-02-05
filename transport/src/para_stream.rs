use std::{sync::{Weak, RwLock}, thread::JoinHandle};

use atomic_buffer::{AtomicBuffer, AtomicBufferWeak, error::BufferError};
use poison_error_obj::PoisonErrorObj;

use self::{error::{JoinThreadError, SpawnThreadError}, transport::{StreamTransport, Transport}};

use super::*;

/// A stream running in a parallel process (another thread)
#[derive(Debug)]
pub struct ParaStream<RequestType, ResponseType, TransportType, BufferReceive = AtomicBuffer<Result<ResponseType, <TransportType as Transport>::MessageError>>>
where
    TransportType: StreamTransport<RequestType, ResponseType>,
    BufferReceive: ReceiveBuffer<RequestType, ResponseType, TransportType>
{
    name: String,
    target: TransportType::Target,
    id: TransportType::Target,
    transport: Weak<RwLock<TransportType>>,
    thread: Option<JoinHandle<TransportType::StreamError>>,
    buffer_send: AtomicBuffer<RequestType>,
    buffer_receive: BufferReceive
}

impl<MI, MO, T> ParaStream<MI, MO, T, AtomicBuffer<Result<MO, <T as Transport>::MessageError>>>
where
    T: StreamTransport<MI, MO> + 'static,
    MI: Send + Sync + 'static,
    MO: Send + Sync + 'static
{
    pub fn new(name: &str, target: T::Target, transport: Weak<RwLock<T>>)
        -> Result<Self, T::SpawnStreamError>
    {
        let buffer_send = AtomicBuffer::new();
        let buffer_receive = AtomicBuffer::new();
        let (id, thread) = ParaStream::new_thread(
            name,
            target,
            transport.clone(),
            buffer_send.downgrade(),
            buffer_receive.downgrade()
        )?;
        Ok(Self {
            name: name.to_string(),
            target,
            id,
            transport,
            thread: Some(thread),
            buffer_send,
            buffer_receive
        })
    }

    pub fn receive(&self) -> Result<Option<Result<MO, T::MessageError>>, T::StreamError>
    {
        Ok(self.buffer_receive.pop_front().map_err(BufferError::from)?)
    }
}

impl<MI, MO, T, B> ParaStream<MI, MO, T, B>
where
    T: StreamTransport<MI, MO> + 'static,
    MI: Send + Sync + 'static,
    MO: Send + Sync + 'static,
    B: ReceiveBuffer<MI, MO, T> + 'static
{
    pub fn new_from_connection(
        name: &str,
        target: T::Target,
        id: T::Target,
        args: T::StreamArgs,
        transport: Weak<RwLock<T>>,
        buffer_receive: B
    ) -> Result<Self, T::SpawnStreamError>
    {
        let buffer_send = AtomicBuffer::new();
        let thread = Self::new_thread_from_connection(
            name,
            target,
            id,
            args,
            transport.clone(),
            buffer_send.downgrade(),
            buffer_receive.clone()
        )?;
        Ok(Self {
            name: name.to_string(),
            target,
            id,
            transport,
            thread: Some(thread),
            buffer_send,
            buffer_receive
        })
    }

    fn new_thread(
        name: &str,
        target: T::Target,
        transport: Weak<RwLock<T>>,
        buffer_send: AtomicBufferWeak<MI>,
        buffer_receive: B
    )
        -> Result<(T::Target, JoinHandle<T::StreamError>), T::SpawnStreamError>
    {
        let (id, args) = T::connect_stream(target)?;
        Self::new_thread_from_connection(name, target, id, args, transport, buffer_send, buffer_receive)
            .map(|thread| (id, thread))
    }
    
    fn new_thread_from_connection(
        name: &str,
        target: T::Target,
        id: T::Target,
        mut args: T::StreamArgs,
        transport: Weak<RwLock<T>>,
        buffer_send: AtomicBufferWeak<MI>,
        buffer_receive: B
    )
        -> Result<JoinHandle<T::StreamError>, T::SpawnStreamError>
    {
        let mut builder = std::thread::Builder::new()
            .name(format!("{} ({} Stream to {} on {})", name, T::NAME, target, id));
        if let Some(stack_size) = T::LISTENER_STACK_SIZE
        {
            builder = builder.stack_size(stack_size)
        }
        Ok(builder
            .spawn(move || loop {
                match T::stream_loop(
                    &transport,
                    &buffer_send,
                    &buffer_receive,
                    &mut args
                )
                {
                    Ok(()) => (),
                    Err(error) => break error
                }
            })
            .map_err(|error| SpawnThreadError(error))?)
    }

    pub fn get_target(&self) -> T::Target
    {
        self.target
    }
    pub fn get_id(&self) -> T::Target
    {
        self.id
    }
    pub fn get_transport(&self) -> &Weak<RwLock<T>>
    {
        &self.transport
    }

    pub fn check_thread(&mut self) -> Result<(), T::StreamError>
    {
        if let Some(thread) = &self.thread && thread.is_finished()
        {
            let thread = self.thread.take().unwrap();
            return Err(thread
                .join()
                .map_err(|error| JoinThreadError(error))?
                .into()
            )
        }
        Ok(())
    }

    pub fn send(&self, message: MI) -> Result<(), T::StreamError>
    where
        T::StreamError: From<BufferError>
    {
        Ok(self.buffer_send.push_back(message).map_err(Into::into)?)
    }
}