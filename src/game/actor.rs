pub mod server;
pub mod client;
pub mod any;

pub use server::*;
pub use client::*;
pub use any::*;

use crate::player::{PlayerDecisionError, Player};

use super::{Choice, Session, SessionJoinError};

pub type Port = u16;

pub trait Actor
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, SessionJoinError>;
    fn player_make_decision(self: &mut Self, uid: Port) -> Result<Option<Choice>, PlayerDecisionError>;
    fn is_host(self: &Self) -> bool;
}