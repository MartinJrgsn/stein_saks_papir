use std::fmt::Display;


#[derive(Debug)]
pub enum SessionKind
{
    Server,
    Client
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