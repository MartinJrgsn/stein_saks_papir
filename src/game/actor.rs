pub mod server;
pub mod client;
pub mod any;

pub use server::*;
pub use client::*;
pub use any::*;

use super::*;

pub type Port = u16;

pub trait Actor
{
    fn try_join(self: &mut Self, name: &str) -> Result<Port, RequestJoinError>;
    fn player_make_decision(self: &mut Self, uid: Port) -> Result<Option<Choice>, PlayerDecisionError>;
    fn is_host(self: &Self) -> bool;
}