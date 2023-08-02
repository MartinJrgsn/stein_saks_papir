use std::{sync::{RwLock, Weak}, thread::JoinHandle, collections::{HashMap, VecDeque}};

use atomic_buffer::{AtomicBuffer, AtomicBufferWeak, error::BufferError};

use super::*;

pub struct ParaListener<MessageType, TransportType>
where
    MessageType: Send + Sync,
    TransportType: ListenerTransport<MessageType>
{
    target: TransportType::Target,
    id: TransportType::Target,
    transport: Weak<RwLock<TransportType>>,
    thread: JoinHandle<TransportType::ListenerError>,
    buffer_incoming: AtomicBuffer<(TransportType::Target, Result<ParaStream<MessageType, TransportType, ReceiveBufferShare<MessageType, TransportType>>, TransportType::ConnectError>)>,
    buffer_receive: AtomicBuffer<(TransportType::Target, Result<MessageType, TransportType::MessageError>)>,
    connections: HashMap<TransportType::Target, ParaStream<MessageType, TransportType, ReceiveBufferShare<MessageType, TransportType>>>
}
impl<M, T> ParaListener<M, T>
where
    T: ListenerTransport<M> + 'static,
    M: Send + Sync + Clone + 'static
{
    pub fn new(name: &str, id: T::Target, transport: Weak<RwLock<T>>)
        -> Result<Self, T::SpawnListenerError>
    {
        let buffer_incoming = AtomicBuffer::new();
        let buffer_receive = AtomicBuffer::new();
        let connections = HashMap::new();
        let (target, thread) = Self::new_thread(
            name,
            id,
            transport.clone(),
            buffer_incoming.downgrade(),
            buffer_receive.downgrade()
        )?;
        Ok(Self {
            target,
            id,
            transport,
            thread,
            buffer_incoming,
            buffer_receive,
            connections
        })
    }
    fn new_thread(
        name: &str,
        id: T::Target,
        transport: Weak<RwLock<T>>,
        buffer_incoming: AtomicBufferWeak<(T::Target, Result<ParaStream<M, T, ReceiveBufferShare<M, T>>, T::ConnectError>)>,
        buffer_receive: AtomicBufferWeak<(T::Target, Result<M, T::MessageError>)>
    )
        -> Result<(T::Target, JoinHandle<T::ListenerError>), T::SpawnListenerError>
    {
        let (target, mut args) = T::bind_listener(id)?;
        let mut builder = std::thread::Builder::new()
            .name(format!("{} ({} Listener on {})", name, T::NAME, target));
        if let Some(stack_size) = T::LISTENER_STACK_SIZE
        {
            builder = builder.stack_size(stack_size)
        }
        Ok((target, builder
            .spawn(move || loop
            {
                match T::listener_loop(
                    &transport,
                    &buffer_incoming,
                    &buffer_receive,
                    &mut args
                )
                {
                    Ok(()) => (),
                    Err(error) => break error
                }
            })
            .map_err(|error| SpawnThreadError(error))?))
    }

    pub fn get_target(&self) -> T::Target
    {
        self.target
    }
    pub fn get_connected_ids(&self) -> Vec<T::Target>
    {
        self.connections.keys().map(|addr| *addr).collect()
    }
    pub fn get_id(&self) -> T::Target
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

    pub fn update_connections<'a>(&'a mut self) -> (Vec<(T::Target, OnConnect<M, T>)>, Result<(), T::ListenerError>)
    where
        T::ListenerError: From<BufferError>
    {
        let mut events = vec![];

        while let Some((addr, connection)) = match self.buffer_incoming.pop_front()
        {
            Ok(c) => c,
            Err(error) => return (events, Err(BufferError::from(error).into())) 
        }
        {
            if self.connections.contains_key(&addr)
            {
                if let Ok(connection) = connection
                {
                    self.connections.insert(addr, connection);
                    events.push((addr, OnConnect::NewConnection));
                }
            }
            else
            {
                events.push((addr, match connection
                {
                    Ok(connection) => OnConnect::DuplicateConnection(connection),
                    Err(error) => OnConnect::ConnectError(error)
                }))
            }
        }

        (events, Ok(()))
    }

    pub fn disconnect(&mut self, target: &T::Target) -> Option<ParaStream<M, T, ReceiveBufferShare<M, T>>>
    {
        self.connections.remove(target)
    }

    pub fn disconnect_all(&mut self) -> Vec<(T::Target, ParaStream<M, T, ReceiveBufferShare<M, T>>)>
    {
        self.connections.drain().collect()
    }

    pub fn send<'a>(&'a self, target: T::Target, message: M) -> Result<(), T::ListenerError>
    where
        T::StreamError: From<BufferError>,
        T::ListenerError: From<T::StreamError>
    {
        match self.connections.get(&target)
        {
            Some(connection) => {
                connection.send(message).map_err(Into::into)
            },
            None => Err(T::missing_connection_error(target))
        }
    }

    pub fn receive<'a>(&'a self)
        -> Result<Option<(T::Target, Result<M, T::MessageError>)>, T::ListenerError>
    where
        T::ListenerError: From<BufferError>
    {
        Ok(self.buffer_receive.pop_front().map_err(Into::into)?)
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