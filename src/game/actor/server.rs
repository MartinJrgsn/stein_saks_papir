pub mod message;
pub mod error;

pub use message::*;
pub use error::*;

use std::{net::TcpListener, io::{Read, Write}, sync::{RwLock, Arc}};

use crate::game::{SessionJoinError, TryDeserializeTcp, SerializeTcp};

use super::*;

pub struct ActorServer
{
    port: Port,
    uids: Arc<RwLock<Vec<Port>>>,
    send_queue: Arc<RwLock<Vec<ServerMessage>>>,
}

impl ActorServer
{
    pub fn new(port: Port) -> Self
    {
        Self
        {
            port,
            uids: Arc::new(RwLock::new(vec![])),
            send_queue: Arc::new(RwLock::new(vec![]))
        }
    }
    pub fn atomic_copy(&self) -> Self
    {
        Self
        {
            port: self.port,
            uids: self.uids.clone(),
            send_queue: self.send_queue.clone()
        }
    }
    pub fn listen(&self, listener: TcpListener)
    {
        for stream in listener.incoming()
        {
            let mut stream = stream.expect("Invalid stream!");
            let from_port = stream.local_addr().expect("Unable to fetch address").port();
    
            let mut message = vec![];
            stream.read(&mut message).expect("Unable to read");

            match ClientMessage::try_from_tcp_message(&message)
            {
                Ok(message) => match self.handle_message(message, from_port)
                {
                    Ok(Some(response)) => {
                        stream.write_all(&response.into_tcp_message()).expect("Unable to write");
                    },
                    Err(error) => {
                        
                    },
                    _ => ()
                },
                Err(error) => {
                    todo!()
                }
            };
        }
    }
    fn handle_message(&self, message: ClientMessage, from_port: Port) -> Result<Option<ServerMessage>, ActorServerHandleMessageError>
    {
        match message
        {
            ClientMessage::Name(name) => Ok(Some(ServerMessage::JoinResponse(
                self.try_join_from_port(from_port, &name)?
            ))),
            _ => todo!()
        }
    }

    fn try_join_from_port(self: &Self, port: Port, name: &str) -> Result<Port, SessionJoinError>
    {
        let mut uids = self.uids.write()?;
        if uids.contains(&port)
        {
            return Err(SessionJoinError::AlreadyJoined)
        }
        uids.push(port);
        Ok(port)
    }
}

impl Actor for ActorServer
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, SessionJoinError>
    {
        self.try_join_from_port(self.port, name)
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