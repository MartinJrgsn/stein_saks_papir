pub mod human;

pub use human::*;
use crate::game::Choice;
pub trait Player
{
    fn is_human(self: &Self) -> bool
    {
        false
    }

    fn make_decision(self: &mut Self, player_names: [String; 2], choice_log: &[[Choice; 2]]) -> Result<Option<Choice>, PlayerDecisionError>;

    fn get_name(self: &Self) -> &str;
}

pub enum PlayerDecisionError
{
    Quit
}