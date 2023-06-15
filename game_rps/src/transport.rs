pub mod listener;
pub mod transport_tcp_udp;

pub use listener::*;
pub use transport_tcp_udp::*;

use std::{net::SocketAddr, thread::JoinHandle};

use crate::game::SessionKind;

pub type Port = u16;

pub struct SpawnThreadError(std::io::Error);

pub trait TransportObj
{
    
}
pub trait Transport: TransportObj
{
    type StreamError: From<SpawnThreadError>;
    type ListenerError: From<SpawnThreadError>;
    type SpawnStreamError;
    type SpawnListenerError;
    type DeserializeError;
    
    type ListenerType;

    const NAME: &'static str;
    const LISTENER_STACK_SIZE: Option<usize> = None;
    const STREAM_STACK_SIZE: Option<usize> = None;

    fn new_listener(port: Port) -> Result<(SocketAddr, Self::ListenerType), Self::SpawnListenerError>;
    fn new_listener_thread(port: Port, session_kind: SessionKind, listen: impl FnOnce(Self::ListenerType) -> Self::ListenerError) -> Result<JoinHandle<Self::ListenerError>, Self::SpawnListenerError>
    {
        let (target, listener) = Self::new_listener(port)?;
        let mut builder = std::thread::Builder::new()
            .name(format!("{} {} Session Listener on {}", Self::NAME, session_kind, target));
        if let Some(stack_size) = Self::LISTENER_STACK_SIZE
        {
            builder = builder.stack_size(stack_size)
        }
        Ok(builder
            .stack_size(Self::LISTENER_STACK_SIZE)
            .spawn(move || listen(listener))
            .map_err(|error| SpawnThreadError(error))?)
    }
}