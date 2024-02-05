moddef::moddef!(
    flat(pub) mod {
        human
    }
);

use std::{any::Any, fmt::Debug};

use clone_box::clone_box;
use spellcast::{convert::TryConvertInto, downcast::{Downcast, DowncastFrom, DowncastFromRef}, impl_object};

use super::*;

#[clone_box]
pub trait PlayerObj: Any + Send + Sync + Debug + 'static
{
    fn get_name(self: &Self) -> &str;
    fn is_human(self: &Self) -> bool;
    fn as_human(self: &Self) -> Option<&dyn HumanObj>
    {
        None
    }
    fn as_human_mut(self: &mut Self) -> Option<&mut dyn HumanObj>
    {
        None
    }
    fn into_human(self: Box<Self>) -> Result<Box<dyn HumanObj>, Box<dyn PlayerObj>>;
}
impl_object!(PlayerObj);

impl DowncastFromRef<dyn PlayerObj> for dyn HumanObj
{
    fn downcast_from_ref<'a>(from: &'a dyn PlayerObj) -> Option<&'a Self>
    {
        from.as_human()
    }
    fn downcast_from_mut<'a>(from: &'a mut dyn PlayerObj) -> Option<&'a mut Self>
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
        self.into_human()
    }
}