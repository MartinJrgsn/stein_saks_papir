use crate::transport::{Transport, Port};

use super::*;

pub enum SessionEither<TransportType, PlayerType, const PLAYER_COUNT: usize>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    Server(SessionServer<TransportType, PlayerType, PLAYER_COUNT>),
    Client(SessionClient<TransportType>)
}

impl<TransportType, PlayerType, const PLAYER_COUNT: usize> SessionObj for SessionEither<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>
    {
        match self
        {
            Self::Server(session) => session.try_join(name),
            Self::Client(session) => session.try_join(name)
        }
    }

    fn kind(self: &Self) -> SessionKind
    {
        match self
        {
            Self::Server(session) => session.kind(),
            Self::Client(session) => session.kind()
        }
    }

    fn as_host(self: &Self) -> Option<&dyn SessionServerObj>
    {
        match self
        {
            Self::Server(session) => Some(session),
            Self::Client(_) => None
        }
    }

    fn as_host_mut(self: &mut Self) -> Option<&mut dyn SessionServerObj>
    {
        match self
        {
            Self::Server(session) => Some(session),
            Self::Client(_) => None
        }
    }

    fn into_host(self: Box<Self>) -> Result<Box<dyn SessionServerObj>, Box<dyn SessionClientObj>>
    {
        match *self
        {
            Self::Server(session) => Ok(Box::new(session)),
            Self::Client(session) => Err(Box::new(session))
        }
    }

    fn get_local_uid(self: &Self, events: &mut VecDeque<SpinEvent>) -> Port
    {
        match self
        {
            Self::Server(session) => session.get_local_uid(events),
            Self::Client(session) => session.get_local_uid(events)
        }
    }
}
impl<TransportType, PlayerType, const PLAYER_COUNT: usize> GameSessionObj<PlayerType, PLAYER_COUNT>
for SessionEither<TransportType, PlayerType, PLAYER_COUNT>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    TransportType: Transport + ?Sized
{
    fn is_player_local(self: &Self, player: &PlayerType) -> bool
    {
        match self
        {
            Self::Server(session) => session.is_player_local(player),
            Self::Client(session) => session.is_player_local(player)
        }
    }

    fn spin_once(self: &mut Self, events: &mut VecDeque<SpinEvent>) -> Result<(), SpinError<PlayerType>>
    {
        match self
        {
            Self::Server(session) => session.spin_once(events),
            Self::Client(session) => session.spin_once(events)
        }
    }
}