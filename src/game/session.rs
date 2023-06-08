pub mod tcp_udp;
pub mod error;
pub mod event;

pub use tcp_udp::*;
pub use error::*;
pub use event::*;

use super::*;

pub trait Session
{
    fn try_join(self: &mut Self, ui: &mut dyn UI) -> Result<(Port, String), RequestJoinError>;
    fn player_make_decision(self: &mut Self, uid: Port) -> Result<Option<Choice>, PlayerDecisionError>;

    fn is_host(self: &Self) -> bool;
    fn is_user(self: &Self, player: &dyn Player) -> bool;
    fn get_host_player_uid(self: &Self) -> Port;
}