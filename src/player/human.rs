pub mod human_extra;
pub mod human_obj;

pub use human_extra::*;
pub use human_obj::*;

use super::*;

#[derive(Clone, Debug)]
pub struct Human<Extra>
where Extra: HumanExtra, Self: Is<dyn HumanObj>
{
    pub(super) uid: Port,
    pub(super) name: String,
    pub(super) extra: Extra
}

impl<Extra> Human<Extra>
where Extra: HumanExtra
{
    pub fn new(uid: Port, name: String) -> Self
    where Extra: Default
    {
        Self
        {
            uid,
            name,
            extra: Extra::default()
        }
    }
    pub fn get_extra(&self) -> &Extra
    {
        &self.extra
    }
    pub fn get_extra_mut(&mut self) -> &mut Extra
    {
        &mut self.extra
    }
}
impl Human<()>
{
    pub fn add_extra<Extra>(self) -> Human<Extra>
    where Extra: HumanExtra
    {
        Human
        {
            uid: self.uid,
            name: self.name,
            extra: Extra::default()
        }
    }
    fn clone_with_extra<Extra>(&self) -> Human<Extra>
    where Extra: HumanExtra
    {
        Human
        {
            uid: self.uid,
            name: self.name.clone(),
            extra: Extra::default()
        }
    }
}
impl<Extra> PlayerObj for Human<Extra>
where Extra: HumanExtra
{
    fn get_name(&self) -> &str
    {
        &self.name
    }
    fn is_human(self: &Self) -> bool
    {
        true
    }
    fn as_human(self: &Self) -> Option<&dyn HumanObj>
    {
        Some(self)
    }
    fn as_human_mut(self: &mut Self) -> Option<&mut dyn HumanObj>
    {
        Some(self)
    }
    fn into_human(self: Box<Self>) -> Result<Box<dyn HumanObj>, Box<dyn PlayerObj>>
    {
        Ok(self)
    }
}
impl<From, ToExtra, Obj> TryConvertInto<Human<ToExtra>, Obj> for From
where
    From: PlayerObj + Upcast<Obj> + ?Sized,
    Obj: Is<Obj> + Downcast<dyn HumanObj, Obj> + ?Sized,
    ToExtra: HumanExtra
{
    fn try_convert_into(self: Box<Self>) -> Result<Box<Human<ToExtra>>, Box<Obj>>
    {
        Obj::downcast(From::upcast(self)).map(|human| human.convert_into())
    }
}