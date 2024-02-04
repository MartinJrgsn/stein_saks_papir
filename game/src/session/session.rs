
use std::{marker::Unsize, net::IpAddr, collections::VecDeque, fmt::Display};

use transport::{transport::{ListenerTransport, Transport}, ParaListener, ParaStream};

use crate::{game::{Game, GameObj}, message::{ClientMessage, ServerMessage}};

use super::*;

pub enum Session<GameType>
where
    GameType: Game
{
    Client {
        stream: ParaStream<ClientMessage<GameType::ClientMessageData>, ServerMessage<GameType::ServerMessageData, GameType::Target, GameType::MessageError, GameType::GameEndResult>, GameType::TransportType>
    },
    Server {
        listener: ParaListener<ServerMessage<GameType::ServerMessageData, GameType::Target, GameType::MessageError, GameType::GameEndResult>, ClientMessage<GameType::ClientMessageData>, GameType::TransportType>
    }
}
impl<GameType> Session<GameType>
where
    GameType: Game
{
    pub fn kind(&self) -> SessionKind
    {
        match self
        {
            Self::Client { .. } => SessionKind::Client,
            Self::Server { .. } => SessionKind::Server
        }
    }
}