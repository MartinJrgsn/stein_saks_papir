pub mod tcp;
pub mod error;

pub use tcp::*;
pub use error::*;

use super::*;

pub trait Session
{
    fn try_join(self: &mut Self, ui: &mut dyn UI) -> Result<(Port, String), SessionJoinError>;
    fn player_make_decision(self: &mut Self, uid: Port) -> Result<Option<Choice>, PlayerDecisionError>;

    fn is_host(self: &Self) -> bool;
    fn is_user(self: &Self, player: &dyn Player) -> bool;
    fn get_host_player_uid(self: &Self) -> Port;
}