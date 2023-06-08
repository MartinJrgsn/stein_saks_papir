pub mod message;
pub mod friend;

pub use message::*;
pub use friend::*;

use std::{sync::{RwLock, Arc}, time::SystemTime, net::TcpStream, io::{Write, Read}};

use crate::game::{JoinError, RequestError, SendError, SerializeTcp, TryDeserializeTcp, DeserializeTcpError};

use super::*;

pub struct ActorClient
{
    friend: ActorClientFriend
}

impl ActorClient
{
    const JOIN_TIMEOUT_MILLIS: u128 = 1000;

    pub fn new() -> Self
    {
        Self
        {
            friend: ActorClientFriend
            {
                send: Arc::new(RwLock::new(None)),
                response: Arc::new(RwLock::new(None)),
                awaiting_response: Arc::new(RwLock::new(false))
            }
        }
    }
    pub fn new_friend(&self) -> ActorClientFriend
    {
        ActorClientFriend
        {
            send: self.friend.send.clone(),
            response: self.friend.response.clone(),
            awaiting_response: self.friend.awaiting_response.clone()
        }
    }

    fn send(&self, message: ClientMessage) -> Result<(), SendError>
    {
        let begin_time = SystemTime::now();
        while self.friend.send.read()?.is_some()
        {
            if begin_time.elapsed()?.as_millis() >= ActorClient::JOIN_TIMEOUT_MILLIS
            {
                return Err(SendError::Deadlock)
            }
        }
        *self.friend.send.write()? = Some(message);
        Ok(())
    }
    fn request(&self, message: ClientMessage) -> Result<ServerMessage, RequestError>
    {
        self.send(message)?;
        
        *self.friend.awaiting_response.write()? = true;

        let begin_time = SystemTime::now();
        loop
        {
            if let Some(response) = self.friend.response.write()?.take()
            {
                return Ok(response?)
            }
            if begin_time.elapsed()?.as_millis() >= ActorClient::JOIN_TIMEOUT_MILLIS
            {
                return Err(RequestError::Timeout)
            }
        }
    }
}

impl Actor for ActorClient
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>
    {
        match self.request(ClientMessage::Name(name.to_string()))?
        {
            ServerMessage::Response(response) => match response
            {
                ServerResponse::OnJoin(event) => event.into()
            },
            ServerMessage::Error(error) => Err(error.into())
        }
    }

    fn player_make_decision(self: &mut Self, uid: Port) -> Result<Option<Choice>, PlayerDecisionError>
    {
        todo!()
    }

    fn is_host(self: &Self) -> bool
    {
        false
    }
}