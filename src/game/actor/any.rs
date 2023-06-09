use super::*;

pub enum ActorAny<const PLAYER_COUNT: usize>
{
    Server(ActorServer<PLAYER_COUNT>),
    Client(ActorClient)
}

impl<const PLAYER_COUNT: usize> ActorAny<PLAYER_COUNT>
{
    pub fn new(session: &impl Session<PLAYER_COUNT>, port: Port) -> Self
    {
        if session.is_host()
        {
            ActorAny::Server(ActorServer::new(port))
        }
        else
        {
            ActorAny::Client(ActorClient::new())
        }
    }
}

impl<const PLAYER_COUNT: usize> Actor<PLAYER_COUNT> for ActorAny<PLAYER_COUNT>
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>
    {
        match self
        {
            Self::Server(actor) => actor.try_join(name),
            Self::Client(actor) => actor.try_join(name)
        }
    }

    fn is_host(self: &Self) -> bool
    {
        match self
        {
            Self::Server(actor) => actor.is_host(),
            Self::Client(actor) => actor.is_host()
        }
    }
}