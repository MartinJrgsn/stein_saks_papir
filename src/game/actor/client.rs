pub mod message;

pub use message::*;

use std::{sync::{RwLock, Arc}, time::SystemTime};

use crate::game::SessionJoinError;

use super::*;

pub struct ActorClient
{
    send_queue: Arc<RwLock<Vec<ClientMessage>>>,
    awaiting_join: Arc<RwLock<bool>>,
    join_response: Arc<RwLock<Option<Result<Port, SessionJoinError>>>>
}

impl ActorClient
{
    const JOIN_TIMOUT_MILLIS: u128 = 1000;

    pub fn new() -> Self
    {
        Self
        {
            send_queue: Arc::new(RwLock::new(vec![])),
            awaiting_join: Arc::new(RwLock::new(false)),
            join_response: Arc::new(RwLock::new(None))
        }
    }
}

impl Actor for ActorClient
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, SessionJoinError>
    {
        *self.awaiting_join.write()? = true;
        *self.join_response.write()? = None;
        self.send_queue.write()?.push(ClientMessage::Name(name.to_string()));

        let begin_time = SystemTime::now();
        loop
        {
            if let Some(join_response) = self.join_response.write()?.take()
            {
                return join_response
            }
            if begin_time.elapsed()?.as_millis() >= ActorClient::JOIN_TIMOUT_MILLIS
            {
                return Err(SessionJoinError::Timeout)
            }
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