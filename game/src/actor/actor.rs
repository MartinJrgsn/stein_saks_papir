
use std::{marker::Unsize, net::IpAddr, collections::VecDeque, fmt::Display};

use transport::{transport::{ListenerTransport, StreamTransport, Transport}, ParaListener, ParaStream};

use crate::{game::{Game, GameObj}, message::{ClientMessage, ServerMessage}};

use super::*;

pub enum Actor<ClientMessage, ServerMessage, TransportType>
where
    ClientMessage: Send + Sync,
    ServerMessage: Send + Sync,
    TransportType: StreamTransport<ClientMessage, ServerMessage> + ListenerTransport<ServerMessage, ClientMessage>
{
    Client {
        stream: ParaStream<ClientMessage, ServerMessage, TransportType>
    },
    Server {
        listener: ParaListener<ServerMessage, ClientMessage, TransportType>
    }
}
impl<ClientMessage, ServerMessage, TransportType> Actor<ClientMessage, ServerMessage, TransportType>
where
    ClientMessage: Send + Sync,
    ServerMessage: Send + Sync,
    TransportType: StreamTransport<ClientMessage, ServerMessage> + ListenerTransport<ServerMessage, ClientMessage>
{
    pub fn kind(&self) -> ActorKind
    {
        match self
        {
            Self::Client { .. } => ActorKind::Client,
            Self::Server { .. } => ActorKind::Server
        }
    }
}