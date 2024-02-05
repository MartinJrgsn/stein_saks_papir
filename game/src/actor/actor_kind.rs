use std::fmt::Display;


#[derive(Debug)]
pub enum ActorKind
{
    Server,
    Client
}
impl Display for ActorKind
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