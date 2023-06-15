pub mod human;

use std::{fmt::Debug, ops::CoerceUnsized};

pub use human::*;

use super::*;

pub trait PlayerObj: AsAny + Send + Sync + Debug + 'static
{
    fn get_name(self: &Self) -> &str;
    fn is_human(self: &Self) -> bool;
    fn as_human(self: &Self) -> Option<&dyn HumanObj>;
    fn as_human_mut(self: &mut Self) -> Option<&mut dyn HumanObj>;
    fn into_human(self: Box<Self>) -> Result<Box<dyn HumanObj>, Box<dyn PlayerObj>>;
}
Object!(PlayerObj);

impl DowncastFromRef<dyn PlayerObj> for dyn HumanObj
{
    fn downcast_from_ref(from: &dyn PlayerObj) -> Option<&Self>
    {
        from.as_human()
    }

    fn downcast_from_mut(from: &mut dyn PlayerObj) -> Option<&mut Self>
    {
        from.as_human_mut()
    }
}
impl DowncastFrom<dyn PlayerObj, dyn PlayerObj> for dyn HumanObj
{
    fn downcast_from(from: Box<dyn PlayerObj>) -> Result<Box<Self>, Box<dyn PlayerObj>>
    {
        from.into_human()
    }
}
impl TryConvertInto<dyn HumanObj, dyn PlayerObj> for dyn PlayerObj
{
    fn try_convert_into(self: Box<Self>) -> Result<Box<dyn HumanObj>, Box<dyn PlayerObj>>
    {
        self.downcast()
    }
}