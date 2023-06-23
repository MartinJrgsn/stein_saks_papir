use std::{sync::{RwLock, Weak, MutexGuard, PoisonError}, thread::JoinHandle, collections::VecDeque};

use atomic_buffer::{AtomicBuffer, AtomicBufferWeak};

use super::*;

pub struct ParaListener<MessageType, TransportType>
where
    TransportType: ListenerTransport<MessageType>
{
    target: TransportType::Target,
    id: TransportType::Id,
    transport: Weak<RwLock<TransportType>>,
    thread: JoinHandle<TransportType::ListenerError>,
    buffer: AtomicBuffer<(TransportType::Id, Result<MessageType, TransportType::DeserializeError>)>
}
impl<M, T> ParaListener<M, T>
where
    T: ListenerTransport<M> + 'static,
    M: Send + Sync + Clone + 'static
{
    pub fn new(name: &str, id: T::Id, transport: Weak<RwLock<T>>)
        -> Result<Self, T::SpawnListenerError>
    {
        let buffer = AtomicBuffer::new();
        let (target, thread) = Self::new_thread(
            name,
            id,
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
        id: T::Id,
        transport: Weak<RwLock<T>>,
        buffer: AtomicBufferWeak<(T::Id, Result<M, T::DeserializeError>)>
    )
        -> Result<(T::Target, JoinHandle<T::ListenerError>), T::SpawnListenerError>
    {
        let (target, args) = T::new_listener_args(id)?;
        let mut builder = std::thread::Builder::new()
            .name(format!("{} ({} Listener on {})", name, T::NAME, target));
        if let Some(stack_size) = T::LISTENER_STACK_SIZE
        {
            builder = builder.stack_size(stack_size)
        }
        Ok((target, builder
            .spawn(move || T::listener_loop(
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

    pub fn check_thread(self) -> Result<Self, T::ListenerError>
    where
        T::ListenerError: From<JoinError>
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

    pub fn receive<'a>(&'a self)
        -> Result<Option<(T::Id, Result<M, T::DeserializeError>)>, T::ListenerError>
    where
        T::ListenerError: From<PoisonError<MutexGuard<'a, VecDeque<(T::Id, Result<M, T::DeserializeError>)>>>>
    {
        Ok(self.buffer.pop_front()?)
    }
}

/*impl<MessageType> Listen<TransportTcpUdp> for Listener<MessageType>
where
    MessageType: TryDeserializeTcp
{
    fn listen(&self, listener: TcpListener) -> <TransportTcpUdp as Transport>::ListenerError
    {
        for stream in listener.incoming()
        {
            if let Err(error) = self.on_incoming_tcp(stream)
            {
                return error;
            }
        }
        TcpThreadError::ListenerStopped
    }
}
impl<MessageType> Listener<MessageType>
where
    Self: Listen<TransportTcpUdp>,
    MessageType: TryDeserializeTcp
{
    fn on_incoming_tcp(&self, stream: Result<TcpStream, std::io::Error>)
        -> Result<(), <TransportTcpUdp as Transport>::ListenerError>
    {
        let mut stream = stream
            .map_err(|error| TcpThreadError::InvalidStream(error))?;

        let from_port = stream.local_addr()
            .map_err(|error| TcpThreadError::CannotRetrieveAddressFromStream(error))?
            .port();
        
        let mut message_bytes = vec![];
        stream.read(&mut message_bytes)
            .map_err(|error| TcpThreadError::ReadStreamError(error))?;

        let message = MessageType::try_deserialize_tcp(&message_bytes)?;

        self.receive_buffer.write()?.push((from_port, message));
        
        /*stream.write(&response.serialize_tcp())
            .map_err(|error| TcpThreadError::WriteStreamError(error))?;*/
    }
}*/