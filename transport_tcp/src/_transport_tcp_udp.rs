pub mod error;
pub mod serialize;
pub mod deserialize;

pub use error::*;
pub use serialize::*;
pub use deserialize::*;

use crate::transport::{TransportObj, Transport, Port};

use super::*;

use std::{net::{SocketAddr, IpAddr, TcpListener, TcpStream}, thread::JoinHandle, time::Duration};

pub struct TransportTcpUdp
{

}
impl TransportObj for TransportTcpUdp
{

}
impl Transport for TransportTcpUdp
{
    type StreamError = TcpThreadError;
    type ListenerError = TcpThreadError;
    type SpawnStreamError = SpawnTcpStreamError;
    type SpawnListenerError = SpawnTcpListenerError;
    type DeserializeError = DeserializeTcpError;
    
    type ListenerType = TcpListener;

    const NAME: &'static str = "TCP/UDP";

    fn new_listener(port: Port) -> Result<(SocketAddr, Self::ListenerType), Self::SpawnListenerError>
    {
        let ip = Session::get_my_ip()?;
        let target = SocketAddr::new(ip, port);

        Ok((target, TcpListener::bind(target).map_err(|error| SpawnTcpListenerError::BindError(error))?))
    }
}

/*
pub struct SessionTcpUdp<PlayerType, const PLAYER_COUNT: usize>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    target: SocketAddr,
    tcp_thread: JoinHandle<TcpThreadError>,
    actor: ActorAny<PlayerType, PLAYER_COUNT>
}


impl<PlayerType, const PLAYER_COUNT: usize> SessionTcpUdp<PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    pub fn new(ip: Option<IpAddr>, port: u16, timeout: Duration) -> Result<Self, NewSessionTcpError>
    {
        Ok(match ip
        {
            Some(ip) => Self::new_client(SocketAddr::new(ip, port), timeout)?,
            None => Self::new_host(port, timeout)?
        })
    }

    pub fn get_my_ip() -> Result<IpAddr, local_ip_address::Error>
    {
        local_ip_address::local_ip()
    }
}

impl<PlayerType, const PLAYER_COUNT: usize> SessionObj for SessionTcpUdp<PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    fn try_join(self: &mut Self, ui: &mut dyn UIRps) -> Result<(Port, String), RequestJoinError>
    {
        let name = ui.promt_for_name(None)?;

        self.actor.try_join(&name)
            .map(|uid| (uid, name))
    }

    fn is_host(self: &Self) -> bool
    {
        self.actor.is_host()
    }
    fn get_local_uid(self: &Self) -> Port
    {
        self.target.port()
    }
}

impl<PlayerType, const PLAYER_COUNT: usize> GameSessionObj<PlayerType, PLAYER_COUNT> for SessionTcpUdp<PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    fn get_actor(self: &Self) -> &ActorAny<PlayerType, PLAYER_COUNT>
    {
        &self.actor
    }
    fn is_player_local(self: &Self, player: &PlayerType) -> bool
    {
        match player.as_human()
        {
            Some(human) => self.get_local_uid() == human.get_uid(),
            None => self.actor.is_host()
        }
    }
}*/