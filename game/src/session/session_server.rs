pub mod message;
pub mod error;
mod friend;
pub mod connection;

pub use message::*;
pub use error::*;
use friend::*;
pub use connection::*;

use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, sync::{RwLock, Arc, RwLockWriteGuard, RwLockReadGuard}, time::{SystemTime, Duration}, collections::{HashMap, VecDeque}, thread::JoinHandle, marker::PhantomData};

use crate::{game::JoinError, transport::{Transport, Port}};

use super::*;

pub struct SessionServer<TransportType, PlayerType, const PLAYER_COUNT: usize>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    timeout: Duration,
    port: Port,
    players: [Option<Box<PlayerType>>; PLAYER_COUNT],
    listener_thread: Option<JoinHandle<TransportType::ListenerError>>,
    friend: SessionServerFriend<TransportType>,
}

impl<TransportType, PlayerType, const PLAYER_COUNT: usize>
    SessionServer<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    pub fn new(port: Port, timeout: Duration) -> Self
    {
        Self
        {
            timeout,
            port,
            players: [(); PLAYER_COUNT].map(|_| None),
            listener_thread: None,
            friend: SessionServerFriend
            {
                join: Arc::new(RwLock::new(vec![])),
                join_response: Arc::new(RwLock::new(HashMap::new())),
                send: Arc::new(RwLock::new(vec![])),
                phantom_data: PhantomData
            }
        }
    }
    fn new_friend(&self) -> SessionServerFriend<TransportType>
    {
        SessionServerFriend
        {
            join: self.friend.join.clone(),
            join_response: self.friend.join_response.clone(),
            send: self.friend.send.clone(),
            phantom_data: PhantomData
        }
    }

    pub fn get_players(&self) -> [Option<&Box<PlayerType>>; PLAYER_COUNT]
    {
        self.players.iter().map(|player| player.as_ref()).next_chunk().unwrap()
    }
    pub fn get_players_mut(&self) -> [Option<&mut Box<PlayerType>>; PLAYER_COUNT]
    {
        self.players.iter().map(|player| player.as_mut()).next_chunk().unwrap()
    }
    pub fn spin_until_get_players(&mut self, events: &mut VecDeque<SpinEvent>) -> (Vec<SpinEvent>, Result<[&Box<PlayerType>; PLAYER_COUNT], SpinError<PlayerType>>)
    {
        loop
        {
            if let Ok(players) = self.get_players().try_map(|player| player.ok_or(()))
            {
                return Ok(players)
            }
            self.spin_once(events)?
        }
    }
    pub fn spin_until_get_players_mut(&mut self, events: &mut VecDeque<SpinEvent>) -> Result<[&mut Box<PlayerType>; PLAYER_COUNT], SpinError<PlayerType>>
    {
        loop
        {
            if let Ok(players) = self.get_players_mut().try_map(|player| player.ok_or(()))
            {
                return Ok(players)
            }
            self.spin_once(events)?
        }
    }
    pub fn is_local<E>(&self, human: &Human<E>) -> bool
    where E: HumanExtra
    {
        human.get_uid() == self.port
    }
    fn try_add_player(&mut self, new_player: Box<PlayerType>) -> Result<(), AddPlayerError<PlayerType>>
    {
        for player in self.players.iter_mut()
        {
            if player.is_none()
            {
                *player = Some(new_player);
                return Ok(());
            }
        }
        Err(AddPlayerError::GameFull(new_player))
    }
    fn try_add_some_player(&mut self, new_player: Box<dyn PlayerObj>) -> Result<(), AddSomePlayerError<PlayerType>>
    {
        Ok(self.try_add_player(
            PlayerType::try_convert_from(new_player)
                .map_err(|new_player| AddSomePlayerError::CannotConvert(new_player))?
        )?)
    }
    fn kick_player(&mut self, index: usize) -> Result<Option<Box<PlayerType>>, SendError>
    {
        Ok(match self.players.get_mut(index).and_then(|player| player.take())
        {
            Some(player) => {
                if let Some(human) = <dyn HumanObj>::downcast_from_ref(player.upcast_ref())
                {
                    self.send(ServerMessage::Event(ServerEvent::OnKick(human.get_uid())), human.get_uid())?;
                }
                Some(player)
            },
            None => None
        })
    }
    fn get_or_spawn_listener_thread(&mut self, events: &mut VecDeque<SpinEvent>)
        -> Result<(bool, &mut JoinHandle<TransportType::StreamError>), TransportType::SpawnListenerError>
    {
        match &mut self.listener_thread
        {
            None => {
                let friend = self.new_friend();
                self.listener_thread = Some(TransportType::new_listener_thread(self.port, self.kind(), move |listener| friend.listen(listener))?); 
                events.push_back(SpinEvent::OnSpawnListener);
                Ok((true, self.listener_thread.as_mut().unwrap()))
            },
            Some(listener) => Ok((false, listener))
        }
    }
    
    /*fn send(&self, message: ServerMessage, port: Port) -> Result<(), SendError>
    {
        let begin_time = SystemTime::now();
        while self.friend.send.read()?.is_some()
        {
            if begin_time.elapsed()?.as_millis() >= SessionClient::JOIN_TIMEOUT_MILLIS
            {
                return Err(SendError::Deadlock)
            }
        }
        *self.friend.send.write()? = Some(message);
        Ok(())
    }
    fn request(&self, message: ServerMessage, port: Port) -> Result<ServerMessage, RequestError>
    {
        self.send(message, port)?;
        
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

impl<TransportType, PlayerType, const PLAYER_COUNT: usize> SendMessage<ServerMessage, Port>
for SessionServer<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    
}
impl<TransportType, PlayerType, const PLAYER_COUNT: usize> ReceiveMessage<ClientMessage, Port>
for SessionServer<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    
}
impl<TransportType, PlayerType, const PLAYER_COUNT: usize> SendRequest<ServerMessage, ClientMessage, Port>
for SessionServer<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    
}

impl<TransportType, PlayerType, const PLAYER_COUNT: usize> SessionObj
for SessionServer<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>
    {
        Ok(self.friend.try_join_from_port(self.port, name)??)
    }

    fn get_local_uid(self: &Self, events: &mut VecDeque<SpinEvent>) -> Port
    {
        self.port
    }
}
impl<TransportType, PlayerType, const PLAYER_COUNT: usize> SessionServerObj
for SessionServer<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{

}

impl<TransportType, PlayerType, const PLAYER_COUNT: usize> GameSessionObj<PlayerType, PLAYER_COUNT>
for SessionServer<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized,
    TransportType::StreamError: Into<SpinError<PlayerType>>
{
    fn spin_once(&mut self, events: &mut VecDeque<SpinEvent>) -> Result<(), SpinError<PlayerType>>
    {
        match &self.listener_thread
        {
            None => {
                let (spawned_new, thread) = self.get_or_spawn_listener_thread()?;
                self.listener_thread = ;
                events.push_back(SpinEvent::OnSpawnListener);
            },
            Some(listener) => if listener.is_finished()
            {
                return Err(self.listener_thread.take().unwrap().join()
                    .map_err(|error| SpinError::PoisonError(PoisonError))?
                    .into())
            }
        }

        while let Some(new_player) = self.friend.join.write()?
            .pop()
        {
            if let Err(error) = self.try_add_some_player(new_player)
            {
                let response = ServerMessage::try_from(error)?;
                self.send(response, ())?;
            }
        }
        Ok(())
    }
}