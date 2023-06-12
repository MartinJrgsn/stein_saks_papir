pub mod message;
pub mod error;
pub mod friend;
pub mod connection;

pub use message::*;
pub use error::*;
pub use friend::*;
pub use connection::*;

use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, sync::{RwLock, Arc, RwLockWriteGuard, RwLockReadGuard}};

use crate::game::{JoinError, TryDeserializeTcp, SerializeTcp};

use super::*;

pub struct ActorServer<const PLAYER_COUNT: usize>
{
    port: Port,
    friend: ActorServerFriend<PLAYER_COUNT>,
}

impl<const PLAYER_COUNT: usize> ActorServer<PLAYER_COUNT>
{
    pub type PlayersReadGuard<'a> = RwLockReadGuard<'a, [Option<Box<dyn player::PlayerObj>>; PLAYER_COUNT]>;
    pub type PlayersWriteGuard<'a> = RwLockWriteGuard<'a, [Option<Box<dyn player::PlayerObj>>; PLAYER_COUNT]>;

    pub fn new(port: Port) -> Self
    {
        Self
        {
            port,
            friend: ActorServerFriend
            {
                players: Arc::new(RwLock::new([(); PLAYER_COUNT].map(|_| None))),
                send_queue: Arc::new(RwLock::new(vec![]))
            }
        }
    }
    pub fn new_friend(&self) -> ActorServerFriend<PLAYER_COUNT>
    {
        ActorServerFriend
        {
            players: self.friend.players.clone(),
            send_queue: self.friend.send_queue.clone()
        }
    }

    pub fn get_players(&self) -> Result<([Option<&Box<dyn PlayerObj>>; PLAYER_COUNT], Self::PlayersReadGuard<'_>), PoisonError>
    {
        let guard = self.friend.players.read()?;
        let players = guard.iter().map(|player| player.as_ref()).next_chunk().unwrap();
        Ok((players, guard))
    }
    pub fn get_players_mut(&self) -> Result<([Option<&mut Box<dyn PlayerObj>>; PLAYER_COUNT], Self::PlayersWriteGuard<'_>), PoisonError>
    {
        let mut guard = self.friend.players.write()?;
        let players = guard.iter_mut().map(|player| player.as_mut()).next_chunk().unwrap();
        Ok((players, guard))
    }
    pub fn get_players_or_wait(&self) -> Result<([&Box<dyn PlayerObj>; PLAYER_COUNT], Self::PlayersReadGuard<'_>), PoisonError>
    {
        loop
        {
            let (players, guard) = self.get_players()?;
            if let Ok(players) = players.try_map(|player| player.ok_or(()))
            {
                return Ok((players, guard))
            }
        }
    }
    pub fn get_players_or_wait_mut(&self) -> Result<([&mut Box<dyn PlayerObj>; PLAYER_COUNT], Self::PlayersWriteGuard<'_>), PoisonError>
    {
        loop
        {
            let (mut players, guard) = self.get_players_mut()?;
            if let Ok(players) = players.try_map(|player| player.ok_or(()))
            {
                return Ok((players, guard))
            }
        }
    }
    pub fn is_local<E>(&self, human: &Human<E>) -> bool
    where E: HumanExtra
    {
        human.get_uid() == self.port
    }
}

impl<const PLAYER_COUNT: usize> Actor<PLAYER_COUNT> for ActorServer<PLAYER_COUNT>
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>
    {
        Ok(self.friend.try_join_from_port(self.port, name)??)
    }

    fn is_host(self: &Self) -> bool
    {
        true
    }
}