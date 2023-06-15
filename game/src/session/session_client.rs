pub mod message;
mod friend;

pub use message::*;
use friend::*;

use std::{sync::{RwLock, Arc}, net::TcpStream, io::{Write, Read}, thread::JoinHandle};

use crate::{game::SendError, transport::Port};

use super::*;

pub struct SessionClient<TransportType>
where
    TransportType: Transport + ?Sized
{
    timeout: Duration,
    target: SocketAddr,
    stream: Option<(Port, JoinHandle<TransportType::StreamError>)>,
    friend: SessionClientFriend
}

impl<TransportType> SessionClient<TransportType>
where
    TransportType: Transport + ?Sized
{
    pub fn new(target: SocketAddr, timeout: Duration) -> Self
    {
        Self
        {
            timeout,
            target,
            stream: None,
            friend: SessionClientFriend
            {
                send: Arc::new(RwLock::new(None)),
                response: Arc::new(RwLock::new(None)),
                awaiting_response: Arc::new(RwLock::new(false))
            }
        }
    }
    pub fn new_friend(&self) -> SessionClientFriend
    {
        SessionClientFriend
        {
            send: self.friend.send.clone(),
            response: self.friend.response.clone(),
            awaiting_response: self.friend.awaiting_response.clone()
        }
    }

    /*fn send(&self, message: ClientMessage) -> Result<(), SendError>
    {
        let begin_time = SystemTime::now();
        while self.friend.send.read()?.is_some()
        {
            if begin_time.elapsed()? >= self.timeout
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
            if begin_time.elapsed()?.as_millis() >= SessionClient::JOIN_TIMEOUT_MILLIS
            {
                return Err(RequestError::Timeout)
            }
        }
    }*/
}

impl<TransportType> SendMessage<ClientMessage, ()> for SessionClient<TransportType>
where
    TransportType: Transport + ?Sized
{
    fn is_ready(&self) -> Result<bool, SendError>
    {
        Ok(!self.friend.awaiting_response.read()? && self.friend.send.read()?.is_none())
    }
    unsafe fn send_message_unchecked(&self, to: (), message: ClientMessage) -> Result<(), SendError>
    {
        *self.friend.send.write()? = Some(message);
        Ok(())
    }
}
impl<TransportType> ReceiveMessage<ServerMessage, ()> for SessionClient<TransportType>
where
    TransportType: Transport + ?Sized
{
    type ReceiveError = ReceiveError;

    fn receive_once(&self, from: ()) -> Result<Option<ServerMessage>, ReceiveError>
    {
        Ok(self.friend.response.write()?.take().transpose()?)
    }
}
impl<TransportType> SendRequest<ClientMessage, ServerMessage, ()> for SessionClient<TransportType>
where
    TransportType: Transport + ?Sized
{
    fn on_send_request(&mut self) -> Result<(), RequestError>
    {
        *self.friend.awaiting_response.write()? = true;
        Ok(())
    }
}

impl<TransportType> SessionObj for SessionClient<TransportType>
where
    TransportType: Transport + ?Sized
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>
    {
        match self.request(ClientMessage::Name(name.to_string()))?
        {
            ServerMessage::Event(event) => todo!(),
            ServerMessage::Response(response) => match response
            {
                ServerResponse::OnJoin(event) => event.into()
            },
            ServerMessage::Error(error) => Err(error.into())
        }
    }

    fn is_host(self: &Self) -> bool
    {
        false
    }

    fn as_host(self: &Self) -> Option<&dyn SessionServerObj>
    {
        None
    }

    fn as_host_mut(self: &mut Self) -> Option<&mut dyn SessionServerObj>
    {
        None
    }

    fn into_host(self: Box<Self>) -> Result<Box<dyn SessionServerObj>, Box<dyn SessionClientObj>>
    {
        Err(self)
    }

    fn get_local_uid(self: &Self, events: &mut VecDeque<SpinEvent>) -> Result<Port, TransportType::SpawnStreamError>
    {
        self.get_or_spawn_stream(events)?.0
    }
}
impl<TransportType> SessionClientObj for SessionClient<TransportType>
where
    TransportType: Transport + ?Sized
{

}
impl<TransportType, PlayerType, const PLAYER_COUNT: usize> GameSessionObj<PlayerType, PLAYER_COUNT>
for SessionClient<TransportType>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    fn spin_once(self: &mut Self, events: &mut VecDeque<SpinEvent>) -> Result<(), SpinError<PlayerType>>
    {
        
    }
}