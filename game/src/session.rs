use std::{collections::HashMap, fmt::Debug, sync::{Weak, RwLock}};

use spellcast::downcast::*;
use transport::{event::OnConnect, transport::{ListenerTransport, StreamTransport}, ParaListener, ParaStream};

use crate::{actor::{Actor, ActorKind}, error::{ClientSessionError, JoinError, LeaveError, ServerSessionError, SessionError}, event::{LeaveReason, OnJoin, OnLeave}, game::GameObj, message::{ClientMessage, ServerMessage}, player::{Human, HumanObj, PlayerObj}};

pub struct Session<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
where
    ClientMessageData: Send + Sync,
    ServerMessageData: Send + Sync,
    GameResults: Send + Sync,
    Target: Send + Sync,
    TransportType: StreamTransport<ClientMessage<ClientMessageData>, ServerMessage<ServerMessageData, Target, GameResults>, Target = Target>
        + ListenerTransport<ServerMessage<ServerMessageData, Target, GameResults>, ClientMessage<ClientMessageData>, Target = Target>
{
    pub capacity: usize,
    players: Vec<Box<dyn PlayerObj>>,
    uids: Vec<TransportType::Target>,
    actor: Actor<ClientMessage<ClientMessageData>, ServerMessage<ServerMessageData, Target, GameResults>, TransportType>
}

impl<ClientMessageData, ServerMessageData, Target, GameResults, TransportType> Session<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>
where
    ClientMessageData: Send + Sync + Clone + 'static,
    ServerMessageData: Send + Sync + Clone + 'static,
    GameResults: Send + Sync + Clone + 'static,
    Target: Send + Sync + Clone + 'static + Debug + PartialEq,
    TransportType: StreamTransport<ClientMessage<ClientMessageData>, ServerMessage<ServerMessageData, Target, GameResults>, Target = Target>
        + ListenerTransport<ServerMessage<ServerMessageData, Target, GameResults>, ClientMessage<ClientMessageData>, Target = Target> + 'static
{
    pub fn new(
        capacity: usize,
        actor_type: ActorKind,
        session_name: &str,
        target: Target,
        transport: Weak<RwLock<TransportType>>
    ) -> Result<Self, SessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>>
    {
        Ok(Self {
            capacity,
            players: vec![],
            uids: vec![],
            actor: match actor_type
            {
                ActorKind::Client => Actor::Client {
                    stream: ParaStream::new(session_name, target, transport)
                        .map_err(|error| ClientSessionError::SpawnStreamError(error))?
                },
                ActorKind::Server => Actor::Server {
                    listener: ParaListener::new(session_name, target, transport)
                        .map_err(|error| ServerSessionError::SpawnListenerError(error))?
                }
            }
        })
    }

    pub fn get_target_of_human(&self, human: &dyn HumanObj) -> Option<Target>
    {
        let uid = human.get_uid();
        self.uids.get(uid)
            .map(|target| target.clone())
    }

    pub fn is_target_local(&self, target: Target) -> bool
    {
        match &self.actor
        {
            Actor::Client { stream } => stream.get_id() == target,
            Actor::Server { listener } => listener.get_id() == target,
        }
    }

    pub fn is_server(&self) -> bool
    {
        match &self.actor
        {
            Actor::Client { .. } => false,
            Actor::Server { .. } => true,
        }
    }

    pub fn is_client(&self) -> bool
    {
        match &self.actor
        {
            Actor::Client { .. } => true,
            Actor::Server { .. } => false,
        }
    }

    pub fn is_player_local(&self, player: &dyn PlayerObj) -> bool
    {
        if let Some(human) = player.as_human()
        {
            if let Some(target) = self.get_target_of_human(human)
            {
                self.is_target_local(target)
            }
            else
            {
                // ???
                self.is_server()
            }
        }
        else
        {
            self.is_server()
        }
    }

    fn new_human(uid: usize) -> Human<()>
    {
        Human::new(uid, format!("Human {}", uid))
    }

    fn get_uid_by_target(uids: &[TransportType::Target], target: Target) -> Option<usize>
    {
        uids.iter()
            .enumerate()
            .find_map(|(uid, uid_target)| if *uid_target == target
            {
                Some(uid)
            }
            else
            {
                None
            })
    }

    fn get_or_create_uid_by_target(uids: &mut Vec<TransportType::Target>, target: Target) -> usize
    {
        if let Some(uid) = Self::get_uid_by_target(uids, target.clone())
        {
            uid
        }
        else
        {
            let uid = uids.len();
            uids.push(target);
            uid
        }
    }

    fn get_human_by_uid(players: &[Box<dyn PlayerObj>], uid: usize) -> Option<&dyn HumanObj>
    {
        players.iter()
            .find_map(|player| if let Some(human) = player.as_human()
            {
                if human.get_uid() == uid
                {
                    Some(human)
                }
                else
                {
                    None
                }
            }
            else
            {
                None
            })
    }
    
    fn get_human_by_uid_mut(players: &mut [Box<dyn PlayerObj>], uid: usize) -> Option<&mut dyn HumanObj>
    {
        players.iter_mut()
            .find_map(|player| if let Some(human) = player.as_human_mut()
            {
                if human.get_uid() == uid
                {
                    Some(human)
                }
                else
                {
                    None
                }
            }
            else
            {
                None
            })
    }

    fn get_player_by_uid_mut(players: &mut [Box<dyn PlayerObj>], uid: usize) -> Option<&mut Box<dyn PlayerObj>>
    {
        players.iter_mut()
            .find_map(|player| if let Some(human) = player.as_human_mut()
            {
                if human.get_uid() == uid
                {
                    Some(player)
                }
                else
                {
                    None
                }
            }
            else
            {
                None
            })
    }

    pub fn spin(
        &mut self,
        on_client_data: impl Fn(&mut Box<dyn PlayerObj>, ClientMessageData),
        on_server_data: impl Fn(&mut Box<dyn PlayerObj>, ServerMessageData),
        on_connect: impl Fn(&mut Box<dyn PlayerObj>)
    ) -> Result<(), SessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>>
    {
        match &mut self.actor
        {
            Actor::Server { listener } => {
                listener.check_thread()
                    .map_err(|error| ServerSessionError::ListenerError(error))?;

                let (events, result) = listener.update_connections();

                for (target, event) in events
                {
                    match event
                    {
                        OnConnect::NewConnection => if self.players.len() < self.capacity
                        {
                            let uid = Self::get_or_create_uid_by_target(&mut self.uids, target.clone());
                            if Self::get_human_by_uid(&self.players, uid).is_some()
                            {
                                // Human exists
                                listener.send(target.clone(), ServerMessage::OnJoin(OnJoin::Failure(JoinError::AlreadyJoined)))
                                    .map_err(|error| ServerSessionError::ListenerError(error))?;
                                listener.disconnect(&target);
                            }
                            else
                            {
                                // Human does not exist
                                let human = Self::new_human(uid);
                                self.players.push(Box::new(human));

                                on_connect(self.players.last_mut().unwrap());

                                for client_target in self.uids.iter()
                                {
                                    listener.send(target.clone(), ServerMessage::OnJoin(OnJoin::Success {
                                        server_target: listener.get_target(),
                                        client_target: client_target.clone()
                                    })).map_err(|error| ServerSessionError::ListenerError(error))?;
                                }
                                self.uids.push(target.clone());
                                listener.send_all(ServerMessage::OnJoin(OnJoin::Success {
                                    server_target: listener.get_target(),
                                    client_target: target
                                })).map_err(|error| ServerSessionError::ListenerError(error))?;
                            }
                        }
                        else
                        {
                            listener.send(target.clone(), ServerMessage::OnJoin(OnJoin::Failure(JoinError::GameFull {
                                capacity: self.capacity
                            }))).map_err(|error| ServerSessionError::ListenerError(error))?;
                            listener.disconnect(&target);
                        },
                        OnConnect::DuplicateConnection(stream) => {
                            stream.send(ServerMessage::OnJoin(OnJoin::Failure(JoinError::AlreadyJoined)))
                                .map_err(|error| ServerSessionError::ListenerError(error.into()))?;
                        },
                        OnConnect::ConnectError(error) => return Err(ServerSessionError::ConnectError(error).into()),
                    }
                }

                while let Some((target, message)) = listener.receive()
                    .map_err(|error| ServerSessionError::ListenerError(error))?
                {
                    let message = message.map_err(|error| ServerSessionError::MessageError(Some(error)))?;
                    match message
                    {
                        ClientMessage::Data(data) => {
                            let uid = Self::get_or_create_uid_by_target(&mut self.uids, target);
                            let human = if let Some(human) = Self::get_player_by_uid_mut(&mut self.players, uid)
                            {
                                human
                            }
                            else
                            {
                                let human = Self::new_human(uid);
                                self.players.push(Box::new(human));
                                self.players.last_mut()
                                    .unwrap()
                            };
                            on_client_data(human, data)
                        },
                        ClientMessage::RequestLeave => {
                            if let Some(uid) = Self::get_uid_by_target(&self.uids, target.clone())
                                && let Some(i) = self.players.iter()
                                .enumerate()
                                .find_map(|(i, player)| if let Some(human) = player.as_human()
                                {
                                    if human.get_uid() == uid
                                    {
                                        Some(i)
                                    }
                                    else
                                    {
                                        None
                                    }
                                }
                                else
                                {
                                    None
                                })
                            {
                                self.players.remove(i);

                                listener.send_all(ServerMessage::OnLeave(OnLeave::Success {
                                    server_target: listener.get_id(),
                                    client_target: target.clone(),
                                    reason: LeaveReason::Left
                                })).map_err(|error| ServerSessionError::ListenerError(error))?;
                                listener.disconnect(&target);
                            }
                            else
                            {
                                listener.send(target, ServerMessage::OnLeave(OnLeave::Failure(LeaveError::AlreadyLeft)))
                                    .map_err(|error| ServerSessionError::ListenerError(error))?;
                            }
                        },
                    }
                }

                result.map_err(|error| ServerSessionError::ListenerError(error))?;
            }
            Actor::Client { stream } => {
                stream.check_thread()
                    .map_err(|error| ClientSessionError::StreamError(error))?;

                while let Some(message) = stream.receive()
                    .map_err(|error| ClientSessionError::StreamError(error))?
                {
                    let target = stream.get_id();
                    let message = message.map_err(|error| ClientSessionError::MessageError(Some(error)))?;
                    match message
                    {
                        ServerMessage::Data(data) => {
                            let uid = Self::get_or_create_uid_by_target(&mut self.uids, target);
                            let human = if let Some(human) = Self::get_player_by_uid_mut(&mut self.players, uid)
                            {
                                human
                            }
                            else
                            {
                                let human = Self::new_human(uid);
                                self.players.push(Box::new(human));
                                self.players.last_mut()
                                    .unwrap()
                            };
                            on_server_data(human, data)
                        },
                        ServerMessage::MessageError => return Err(SessionError::ClientError(ClientSessionError::MessageError(None))),
                        ServerMessage::OnJoin(event) => match event
                        {
                            OnJoin::Success {
                                server_target,
                                client_target
                            } => {
                                assert_eq!(server_target, stream.get_target());
                                
                                let uid = Self::get_or_create_uid_by_target(&mut self.uids, client_target.clone());
                                let human = if let Some(human) = Self::get_player_by_uid_mut(&mut self.players, uid)
                                {
                                    human
                                }
                                else
                                {
                                    let human = Self::new_human(uid);
                                    self.players.push(Box::new(human));
                                    self.players.last_mut()
                                        .unwrap()
                                };
                                on_connect(human)
                            }
                            OnJoin::Failure(error) => return Err(ClientSessionError::JoinError(error).into()),
                        },
                        ServerMessage::OnLeave(event) => {

                        },
                        ServerMessage::OnEnd(_) => todo!(),
                    }
                }
            }
        }

        Ok(())
    }

    pub fn await_players(
        &mut self,
        on_client_data: impl Fn(&mut Box<dyn PlayerObj>, ClientMessageData),
        on_server_data: impl Fn(&mut Box<dyn PlayerObj>, ServerMessageData),
        on_connect: impl Fn(&mut Box<dyn PlayerObj>)
    ) -> Result<&mut [Box<dyn PlayerObj>], SessionError<ClientMessageData, ServerMessageData, Target, GameResults, TransportType>>
    {
        while self.players.len() < self.capacity
        {
            self.spin(&on_client_data, &on_server_data, &on_connect)?;
        }

        Ok(self.get_players_mut())
    }

    pub fn get_players(&self) -> &[Box<dyn PlayerObj>]
    {
        &self.players
    }
    
    pub fn get_players_mut(&mut self) -> &mut [Box<dyn PlayerObj>]
    {
        &mut self.players
    }
}