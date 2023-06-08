pub mod message;
pub mod error;
pub mod friend;

pub use message::*;
pub use error::*;
pub use friend::*;

use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, sync::{RwLock, Arc}};

use crate::game::{JoinError, TryDeserializeTcp, SerializeTcp, RequestError};

use super::*;

pub struct ActorServer
{
    port: Port,
    friend: ActorServerFriend
}

impl ActorServer
{
    pub fn new(port: Port) -> Self
    {
        Self
        {
            port,
            friend: ActorServerFriend
            {
                uids: Arc::new(RwLock::new(vec![])),
                send_queue: Arc::new(RwLock::new(vec![]))
            }
        }
    }
    pub fn new_friend(&self) -> ActorServerFriend
    {
        ActorServerFriend
        {
            uids: self.friend.uids.clone(),
            send_queue: self.friend.send_queue.clone()
        }
    }
}

impl Actor for ActorServer
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>
    {
        Ok(self.friend.try_join_from_port(self.port, name)??)
    }

    fn player_make_decision(self: &mut Self, uid: Port) -> Result<Option<Choice>, PlayerDecisionError>
    {
        todo!()
    }

    fn is_host(self: &Self) -> bool
    {
        true
    }
}