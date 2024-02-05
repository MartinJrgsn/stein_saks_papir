use std::{collections::{HashMap, VecDeque}, os::windows::io::AsHandle, sync::{RwLock, Weak}, thread::JoinHandle};

use atomic_buffer::{AtomicBuffer, AtomicBufferWeak, error::BufferError};

use self::{error::{JoinThreadError, SpawnThreadError}, event::OnConnect, transport::ListenerTransport};

use super::*;

pub struct ParaListener<RequestType, ResponseType, TransportType>
where
    RequestType: Send + Sync,
    ResponseType: Send + Sync,
    TransportType: ListenerTransport<RequestType, ResponseType>
{
    name: String,
    target: TransportType::Target,
    id: TransportType::Target,
    transport: Weak<RwLock<TransportType>>,
    thread: Option<JoinHandle<TransportType::ListenerError>>,
    buffer_incoming: AtomicBuffer<(TransportType::Target, Result<ParaStream<RequestType, ResponseType, TransportType, ReceiveBufferShare<RequestType, ResponseType, TransportType>>, TransportType::ConnectError>)>,
    buffer_receive: AtomicBuffer<(TransportType::Target, Result<ResponseType, TransportType::MessageError>)>,
    connections: HashMap<TransportType::Target, ParaStream<RequestType, ResponseType, TransportType, ReceiveBufferShare<RequestType, ResponseType, TransportType>>>
}
impl<MI, MO, T> ParaListener<MI, MO, T>
where
    T: ListenerTransport<MI, MO> + 'static,
    MI: Send + Sync + Clone + 'static,
    MO: Send + Sync + Clone + 'static
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
            name: name.to_string(),
            target,
            id,
            transport,
            thread: Some(thread),
            buffer_incoming,
            buffer_receive,
            connections
        })
    }
    fn new_thread(
        name: &str,
        id: T::Target,
        transport: Weak<RwLock<T>>,
        buffer_incoming: AtomicBufferWeak<(T::Target, Result<ParaStream<MI, MO, T, ReceiveBufferShare<MI, MO, T>>, T::ConnectError>)>,
        buffer_receive: AtomicBufferWeak<(T::Target, Result<MO, T::MessageError>)>
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
    pub fn get_connection_count(&self) -> usize
    {
        self.connections.keys().len()
    }
    pub fn get_id(&self) -> T::Target
    {
        self.id
    }
    pub fn get_transport(&self) -> &Weak<RwLock<T>>
    {
        &self.transport
    }

    pub fn check_thread(&mut self) -> Result<(), T::ListenerError>
    {
        for (_, connection) in self.connections.iter_mut()
        {
            connection.check_thread()?;
        }
        if let Some(thread) = &self.thread && thread.is_finished()
        {
            let thread = self.thread.take().unwrap();
            let error = thread.join()
                .map_err(|error| JoinThreadError(error))?
                .into();
            return Err(error)
        }
        Ok(())
    }

    pub fn update_connections<'a>(&'a mut self) -> (Vec<(T::Target, OnConnect<MI, MO, T>)>, Result<(), T::ListenerError>)
    {
        let mut events = vec![];

        while let Some((addr, connection)) = match self.buffer_incoming.pop_front()
        {
            Ok(c) => c,
            Err(error) => return (events, Err(BufferError::from(error).into())) 
        }
        {
            if !self.connections.contains_key(&addr)
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

    pub fn disconnect(&mut self, target: &T::Target) -> Option<ParaStream<MI, MO, T, ReceiveBufferShare<MI, MO, T>>>
    {
        self.connections.remove(target)
    }

    pub fn disconnect_all(&mut self) -> Vec<(T::Target, ParaStream<MI, MO, T, ReceiveBufferShare<MI, MO, T>>)>
    {
        self.connections.drain()
            .collect()
    }

    pub fn send_all(&self, message: MI) -> Result<(), T::ListenerError>
    {
        for connection in self.connections.values()
        {
            connection.send(message.clone())?;
        }

        Ok(())
    }

    pub fn send(&self, target: T::Target, message: MI) -> Result<(), T::ListenerError>
    {
        match self.connections.get(&target)
        {
            Some(connection) => {
                connection.send(message)
                    .map_err(Into::into)
            },
            None => Err(T::missing_connection_error(target))
        }
    }

    pub fn receive(&self)
        -> Result<Option<(T::Target, Result<MO, T::MessageError>)>, T::ListenerError>
    {
        Ok(self.buffer_receive.pop_front().map_err(BufferError::from)?)
    }

    pub fn receive_from(&self, target: T::Target)
        -> Result<Option<Result<MO, T::MessageError>>, T::ListenerError>
    {
        Ok(self.buffer_receive.filter_pop_front(|(t, _)| *t == target)
            .map_err(BufferError::from)
            .map(|m| m.map(|(_, m)| m))?)
    }
}