pub mod error;
pub mod serialize;
pub mod deserialize;

pub use error::*;
pub use serialize::*;
pub use deserialize::*;

use super::*;

use std::{net::{SocketAddr, IpAddr, TcpListener, TcpStream}, thread::JoinHandle, io::Read};

pub struct SessionTcp
{
    target: SocketAddr,
    thread: JoinHandle<()>,
    actor: ActorAny
}

impl SessionTcp
{
    pub fn new(ip: Option<IpAddr>, port: u16) -> Result<Self, SessionTcpNewError>
    {
        Ok(match ip
        {
            Some(ip) => Self::new_client(SocketAddr::new(ip, port))?,
            None => Self::new_host(port)?
        })
    }

    pub fn new_host(port: u16) -> Result<Self, SessionTcpNewHostError>
    {
        let ip = Self::get_my_ip()?;
        let target = SocketAddr::new(ip, port);
        
        let listener = TcpListener::bind(target)
            .map_err(|error| SessionTcpNewHostError::BindError(error))?;

        let actor = ActorServer::new(port);
        let actor_atomic_copy = actor.atomic_copy();

        let thread = std::thread::Builder::new()
            .name(format!("TCP Session Listener on {}", target))
            //.stack_size(1024)
            .spawn(move || actor_atomic_copy.listen(listener))
            .map_err(|error| SessionTcpNewHostError::SpawnThreadError(error))?;
        
        Ok(Self
        {
            target,
            thread,
            actor: ActorAny::Server(actor)
        })
    }
    pub fn new_client(target: SocketAddr) -> Result<Self, SessionTcpNewClientError>
    {
        let mut stream = TcpStream::connect(target) // Connect to source
            .expect("Unable to connect!");

        todo!()
    }

    fn get_my_ip() -> Result<IpAddr, local_ip_address::Error>
    {
        use local_ip_address::local_ip;

        local_ip()
    }
}

impl Session for SessionTcp
{
    fn try_join(self: &mut Self, ui: &mut dyn UI) -> Result<(Port, String), SessionJoinError>
    {
        let name = ui.promt_for_name(None);

        self.actor.try_join(&name)
            .map(|uid| (uid, name))
    }
    fn player_make_decision(self: &mut Self, uid: Port) -> Result<Option<Choice>, PlayerDecisionError>
    {
        self.actor.player_make_decision(uid)
    }

    fn is_host(self: &Self) -> bool
    {
        self.actor.is_host()
    }
    fn get_host_player_uid(self: &Self) -> Port
    {
        self.target.port()
    }
    fn is_user(self: &Self, player: &dyn Player) -> bool
    {
        match player.as_human()
        {
            Some(human) => self.get_host_player_uid() == human.get_uid(),
            None => false
        }
    }
}