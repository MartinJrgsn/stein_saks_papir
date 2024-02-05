use std::{marker::Unsize, net::SocketAddr};

use spellcast::{convert::{ConvertInto, TryConvert}, downcast::DowncastFrom};

use super::*;

pub trait HumanObj: PlayerObj + Upcast<dyn PlayerObj> + Is<dyn PlayerObj> + Unsize<dyn PlayerObj>
{
    fn get_uid(self: &Self) -> usize;
    fn dump_extra(self: Box<Self>) -> Human<()>;
    fn as_noextra(self: &Self) -> &Human<()>;
}
impl_object!(HumanObj);

impl<Extra> HumanObj for Human<Extra>
where Extra: HumanExtra
{
    fn dump_extra(self: Box<Self>) -> Human<()>
    {
        Human::new(self.uid, self.name)
    }
    fn as_noextra(self: &Self) -> &Human<()>
    {
        unsafe {std::mem::transmute(self)}
    }
    fn get_uid(&self) -> usize
    {
        self.uid
    }
}
impl<FromExtra, ToExtra> ConvertInto<Human<ToExtra>> for Human<FromExtra>
where
    FromExtra: HumanExtra,
    ToExtra: HumanExtra
{
    fn convert_into(self: Box<Self>) -> Box<Human<ToExtra>>
    {
        Box::new(self.as_noextra().clone_with_extra())
    }
}
impl<ToExtra> ConvertInto<Human<ToExtra>> for dyn HumanObj
where
    ToExtra: HumanExtra
{
    fn convert_into(self: Box<Self>) -> Box<Human<ToExtra>>
    {
        Box::new(self.as_noextra().clone_with_extra())
    }
}