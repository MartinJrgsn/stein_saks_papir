pub mod human;

use std::{fmt::Debug, ops::CoerceUnsized};

pub use human::*;

use super::*;

pub trait PlayerObj: AsAny + Send + Sync + Debug
{
    fn get_name(self: &Self) -> &str;
    fn is_human(self: &Self) -> bool;
    fn as_human(self: &Self) -> Option<&dyn HumanObj>;
    fn as_human_mut(self: &mut Self) -> Option<&mut dyn HumanObj>;
    fn into_human(self: Box<Self>) -> Result<Box<dyn HumanObj>, Box<dyn PlayerObj>>;
}
Is!(PlayerObj);
pub trait Player: PlayerObj + Upcast<dyn PlayerObj> + Is<dyn PlayerObj>
{

}