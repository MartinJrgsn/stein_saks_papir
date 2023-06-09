pub mod human;

pub use human::*;

use super::*;

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
    fn get_name(&self) -> &str;
}