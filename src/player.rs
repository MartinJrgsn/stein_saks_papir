pub mod human;

pub use human::*;
use crate::Actor;

use crate::game::{Choice, Session};

pub trait Player
{
    fn is_human(self: &Self) -> bool
    {
        self.as_human().is_some()
    }
    fn as_human(self: &Self) -> Option<&Human>
    {
        None
    }
    fn as_human_mut(self: &mut Self) -> Option<&mut Human>
    {
        None
    }

    fn make_decision(self: &mut Self, player_names: [String; 2], choice_log: &[[Choice; 2]], session: &mut dyn Session) -> Result<Option<Choice>, PlayerDecisionError>;

    fn get_name(&self) -> &str;
}

pub enum PlayerDecisionError
{
    Quit
}