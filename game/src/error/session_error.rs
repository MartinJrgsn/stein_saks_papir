use thiserror::Error;
use transport::transport::{ListenerTransport, StreamTransport};

use crate::message::{ClientMessage, ServerMessage};

use super::JoinError;

#[derive(Error, Debug)]
pub enum SessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
where
    ClientMessageData: Send + Sync,
    ServerMessageData: Send + Sync,
    GameResults: Send + Sync,
    Target: Send + Sync,
    TransportType: StreamTransport<ClientMessage<ClientMessageData>, ServerMessage<ServerMessageData, Target, GameResults>>
        + ListenerTransport<ServerMessage<ServerMessageData, Target, GameResults>, ClientMessage<ClientMessageData>>
{
    ServerError(ServerSessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>),
    ClientError(ClientSessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>)
}

#[derive(Error, Debug)]
pub enum ServerSessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
where
    ClientMessageData: Send + Sync,
    ServerMessageData: Send + Sync,
    GameResults: Send + Sync,
    Target: Send + Sync,
    TransportType: ListenerTransport<ServerMessage<ServerMessageData, Target, GameResults>, ClientMessage<ClientMessageData>>
{
    SpawnListenerError(TransportType::SpawnListenerError),
    ListenerError(TransportType::ListenerError),
    ConnectError(TransportType::ConnectError),
    MessageError(Option<TransportType::MessageError>)
}

impl<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
    From<ServerSessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>>
for
    SessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
where
    ClientMessageData: Send + Sync,
    ServerMessageData: Send + Sync,
    GameResults: Send + Sync,
    Target: Send + Sync,
    TransportType: StreamTransport<ClientMessage<ClientMessageData>, ServerMessage<ServerMessageData, Target, GameResults>>
        + ListenerTransport<ServerMessage<ServerMessageData, Target, GameResults>, ClientMessage<ClientMessageData>>
{
    fn from(error: ServerSessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>) -> Self
    {
        Self::ServerError(error)
    }
}

#[derive(Error, Debug)]
pub enum ClientSessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
where
    ClientMessageData: Send + Sync,
    ServerMessageData: Send + Sync,
    GameResults: Send + Sync,
    Target: Send + Sync,
    TransportType: StreamTransport<ClientMessage<ClientMessageData>, ServerMessage<ServerMessageData, Target, GameResults>>
{
    StreamError(TransportType::StreamError),
    SpawnStreamError(TransportType::SpawnStreamError),
    MessageError(Option<TransportType::MessageError>),
    JoinError(JoinError)
}

impl<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
    From<ClientSessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>>
for
    SessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
where
    ClientMessageData: Send + Sync,
    ServerMessageData: Send + Sync,
    GameResults: Send + Sync,
    Target: Send + Sync,
    TransportType: StreamTransport<ClientMessage<ClientMessageData>, ServerMessage<ServerMessageData, Target, GameResults>>
        + ListenerTransport<ServerMessage<ServerMessageData, Target, GameResults>, ClientMessage<ClientMessageData>>
{
    fn from(error: ClientSessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>) -> Self
    {
        Self::ClientError(error)
    }
}