use super::*;

pub enum ActorAny
{
    Server(ActorServer),
    Client(ActorClient)
}

impl ActorAny
{
    pub fn new(session: &impl Session, port: Port) -> ActorAny
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

impl Actor for ActorAny
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, SessionJoinError>
    {
        match self
        {
            Self::Server(actor) => actor.try_join(name),
            Self::Client(actor) => actor.try_join(name)
        }
    }

    fn player_make_decision(self: &mut Self, uid: Port) -> Result<Option<Choice>, PlayerDecisionError>
    {
        match self
        {
            Self::Server(actor) => actor.player_make_decision(uid),
            Self::Client(actor) => actor.player_make_decision(uid)
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