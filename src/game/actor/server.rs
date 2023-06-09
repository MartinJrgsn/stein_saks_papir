pub mod message;
pub mod error;
pub mod friend;
pub mod connection;

pub use message::*;
pub use error::*;
pub use friend::*;
pub use connection::*;

use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, sync::{RwLock, Arc}, collections::HashMap};

use crate::game::{JoinError, TryDeserializeTcp, SerializeTcp, RequestError};

use super::*;

pub struct ActorServer<const PLAYER_COUNT: usize>
{
    port: Port,
    friend: ActorServerFriend<PLAYER_COUNT>,
}

impl<const PLAYER_COUNT: usize> ActorServer<PLAYER_COUNT>
{
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

    pub fn get_players(&self) -> Result<[Option<&(dyn Player + Send + Sync)>; PLAYER_COUNT], PoisonError>
    {
        let mut players = [None; PLAYER_COUNT];
        let mut iter = self.friend.players.read()?.iter()
            .map(|player|
                player.map(|player| &*player)
            );
        players.fill_with(|| iter.next().unwrap());
        Ok(players)
    }
    pub fn get_players_or_wait(&self) -> Result<[&(dyn Player + Send + Sync); PLAYER_COUNT], PoisonError>
    {
        loop
        {
            /* Better (with nightly):
            if let Ok(players) = self.get_players()?.try_map(|player| player.ok_or(()))
            {
                return Ok(players)
            }
             */

            let players = self.get_players()?;
            if players.iter().all(|player| player.is_some())
            {
                return Ok(players.map(|player| player.unwrap()))
            }
        }
    }
    pub fn is_local(&self, human: &Human) -> bool
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