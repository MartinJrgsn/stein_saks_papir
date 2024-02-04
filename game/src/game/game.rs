use std::time::Duration;

use transport::transport::{ListenerTransport, StreamTransport, Transport};

use crate::{message::{ClientMessage, ServerMessage}, session::Session, ui::UIObj};

use super::*;

pub trait Game: GameObj + Sized
{
    type Target: Send + Sync + Clone + 'static;
    type MessageError: Send + Sync + Clone + 'static;
    type TransportType: ListenerTransport<ServerMessage<Self::ServerMessageData, Self::Target, Self::MessageError, Self::GameEndResult>, ClientMessage<Self::ClientMessageData>, Target = Self::Target, MessageError = Self::MessageError>
        + StreamTransport<ClientMessage<Self::ClientMessageData>, ServerMessage<Self::ServerMessageData, Self::Target, Self::MessageError, Self::GameEndResult>>
        + 'static;
    type ClientMessageData: Send + Sync + Clone + 'static;
    type ServerMessageData: Send + Sync + Clone + 'static;
    type GameEndResult: Send + Sync + Clone + 'static;
    
    fn get_session(self: &Self) -> &Session<Self>;
    fn get_session_mut(self: &mut Self) -> &mut Session<Self>;

    fn new(session: Box<Session<Self>>) -> Self;
    fn on_quit<UIType>(self: Self, ui: &mut UIType) -> Box<Session<Self>>
    where
        UIType: UIObj + ?Sized;
    fn game_loop<UIType>(mut self: Self, ui: &mut UIType) -> (Self::GameEndResult, Box<Session<Self>>)
    where
        UIType: UIObj + ?Sized
    {
        loop
        {
            if let Some(end_result) = self.game_loop_once(ui)
            {
                return (end_result, self.on_quit(ui))
            }
        }
    }
    fn game_loop_once<UIType>(&mut self, ui: &mut UIType) -> Option<Self::GameEndResult>
    where
        UIType: UIObj + ?Sized;
}