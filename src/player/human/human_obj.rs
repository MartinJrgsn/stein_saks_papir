use std::marker::Unsize;

use super::*;

pub trait HumanObj: PlayerObj + Upcast<dyn PlayerObj> + Is<dyn PlayerObj> + Unsize<dyn PlayerObj>
{
    fn get_uid(self: &Self) -> Port;
    fn dump_extra(self: Box<Self>) -> Human<()>;
    fn as_noextra(self: &Self) -> &Human<()>;
}
Object!(HumanObj);
static_assertions::assert_impl_one!(dyn HumanObj: TryConvert<dyn PlayerObj>);

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
    fn get_uid(&self) -> Port
    {
        self.uid
    }
}
impl<From, ToExtra> ConvertInto<Human<ToExtra>> for From
where
    From: HumanObj + Downcast<Human<ToExtra>, dyn HumanObj> + ?Sized,
    ToExtra: HumanExtra,
    Human<ToExtra>: DowncastFrom<From, dyn HumanObj>
{
    fn convert_into(self: Box<Self>) -> Box<Human<ToExtra>>
    {
        self.downcast()
            .unwrap_or_else(|human| Box::new(human.as_noextra().clone_with_extra()))
    }
}