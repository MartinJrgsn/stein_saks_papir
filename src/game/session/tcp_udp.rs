pub mod error;
pub mod serialize;
pub mod deserialize;

pub use error::*;
pub use serialize::*;
pub use deserialize::*;

use super::*;

use std::{net::{SocketAddr, IpAddr, TcpListener, TcpStream}, thread::JoinHandle};

pub struct SessionTcpUdp<const PLAYER_COUNT: usize>
{
    target: SocketAddr,
    tcp_thread: JoinHandle<TcpThreadError>,
    actor: ActorAny<PLAYER_COUNT>
}

impl<const PLAYER_COUNT: usize> SessionTcpUdp<PLAYER_COUNT>
{
    pub fn new(ip: Option<IpAddr>, port: u16) -> Result<Self, NewSessionTcpError>
    {
        Ok(match ip
        {
            Some(ip) => Self::new_client(SocketAddr::new(ip, port))?,
            None => Self::new_host(port)?
        })
    }

    pub fn new_host(port: u16) -> Result<Self, NewSessionTcpHostError>
    {
        let ip = Self::get_my_ip()?;
        let target = SocketAddr::new(ip, port);
        
        let listener = TcpListener::bind(target)
            .map_err(|error| NewSessionTcpHostError::BindError(error))?;

        let actor = ActorServer::new(port);
        let actor_friend = actor.new_friend();

        let tcp_thread = std::thread::Builder::new()
            .name(format!("TCP Host Session Listener on {}", target))
            //.stack_size(1024) //Increase if needed
            .spawn(move || actor_friend.listen_tcp(listener))
            .map_err(|error| NewSessionTcpHostError::SpawnThreadError(error))?;
        
        Ok(Self
        {
            target,
            tcp_thread,
            actor: ActorAny::Server(actor)
        })
    }
    pub fn new_client(target: SocketAddr) -> Result<Self, NewSessionTcpClientError>
    {
        let stream = TcpStream::connect(target)
            .map_err(|error| NewSessionTcpClientError::ConnectError(error))?;
        
        let actor = ActorClient::new();
        let actor_friend = actor.new_friend();

        let tcp_thread = std::thread::Builder::new()
            .name(format!("TCP Client Session Speaker on {}", target))
            //.stack_size(1024) //Increase if needed
            .spawn(move || actor_friend.stream_tcp(stream))
            .map_err(|error| NewSessionTcpClientError::SpawnThreadError(error))?;

        Ok(Self
        {
            target,
            tcp_thread,
            actor: ActorAny::Client(actor)
        })
    }

    pub fn get_my_ip() -> Result<IpAddr, local_ip_address::Error>
    {
        local_ip_address::local_ip()
    }
}

impl<const PLAYER_COUNT: usize> Session<PLAYER_COUNT> for SessionTcpUdp<PLAYER_COUNT>
{
    fn try_join(self: &mut Self, ui: &mut dyn UIRps) -> Result<(Port, String), RequestJoinError>
    {
        let name = ui.promt_for_name(None)?;

        self.actor.try_join(&name)
            .map(|uid| (uid, name))
    }

    fn get_actor(self: &Self) -> &ActorAny<PLAYER_COUNT>
    {
        &self.actor
    }
    fn is_host(self: &Self) -> bool
    {
        self.actor.is_host()
    }
    fn get_local_uid(self: &Self) -> Port
    {
        self.target.port()
    }
    fn is_local(self: &Self, player: &dyn PlayerObj) -> bool
    {
        match player.as_human()
        {
            Some(human) => self.get_local_uid() == human.get_uid(),
            None => self.actor.is_host()
        }
    }
}