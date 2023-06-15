pub mod error;
pub mod event;
pub mod session_server;
pub mod session_client;
pub mod session_either;
pub mod send_message;

use std::{marker::Unsize, net::IpAddr, collections::VecDeque, fmt::Display};

pub use error::*;
pub use event::*;
pub use session_server::*;
pub use session_client::*;
pub use session_either::*;
pub use send_message::*;

use crate::transport::{Transport, Port};

use super::*;

#[derive(Debug)]
pub enum SessionKind
{
    Server,
    Client
}
impl SessionKind
{
    fn is_host(&self) -> bool
    {
        match self
        {
            Self::Server => true,
            Self::Client => false
        }
    }
}
impl Display for SessionKind
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Server => write!(f, "Server"),
            Self::Client => write!(f, "Client")
        }
    }
}

pub struct Session
{

}

impl Session
{
    pub fn get_my_ip() -> Result<IpAddr, local_ip_address::Error>
    {
        local_ip_address::local_ip()
    }
}

pub trait SessionObj
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>;
    fn is_host(self: &Self) -> bool
    {
        self.kind().is_host()
    }
    fn kind(self: &Self) -> SessionKind;

    fn as_host(self: &Self) -> Option<&dyn SessionServerObj>;
    fn as_host_mut(self: &mut Self) -> Option<&mut dyn SessionServerObj>;
    fn into_host(self: Box<Self>) -> Result<Box<dyn SessionServerObj>, Box<dyn SessionClientObj>>;

    fn get_local_uid(self: &Self, events: &mut VecDeque<SpinEvent>) -> Port;
}

pub trait SessionServerObj: SessionObj
{
    
}
default impl<T> SessionObj for T
where
    T: SessionServerObj + Unsize<dyn SessionServerObj> + ?Sized
{
    fn kind(self: &Self) -> SessionKind
    {
        SessionKind::Server
    }
    fn as_host(self: &Self) -> Option<&dyn SessionServerObj>
    {
        Some(self)
    }
    fn as_host_mut(self: &mut Self) -> Option<&mut dyn SessionServerObj>
    {
        Some(self)
    }
    fn into_host(self: Box<Self>) -> Result<Box<dyn SessionServerObj>, Box<dyn SessionClientObj>>
    {
        Ok(self)
    }
}
pub trait SessionClientObj: SessionObj
{

}
impl<T> !SessionClientObj for T
where
    T: SessionServerObj {}

pub trait GameSessionObj<PlayerType, const PLAYER_COUNT: usize>: SessionObj
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    fn is_player_local(self: &Self, player: &PlayerType) -> bool
    {
        if let Some(human) = player.as_human()
        {
            human.get_uid() == self.get_local_uid()
        }
        else
        {
            self.is_host()
        }
    }
    fn spin_once(self: &mut Self, events: &mut VecDeque<SpinEvent>) -> Result<(), SpinError<PlayerType>>;
}