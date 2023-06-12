use super::*;

pub trait HumanObj: PlayerObj
{
    fn get_uid(self: &Self) -> Port;
    fn dump_extra(self: Box<Self>) -> Human<()>;
    fn as_noextra(self: &Self) -> &Human<()>;
}
Is!(HumanObj);
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
    From: HumanObj + ?Sized,
    ToExtra: HumanExtra
{
    fn convert_into(self: Box<Self>) -> Box<Human<ToExtra>>
    {
        if self.as_any().is::<Human<ToExtra>>()
        {
            return self.upcast().downcast().unwrap()
        }
        Box::new(self.as_noextra().clone_with_extra())
    }
}
impl<From, ToExtra> TryConvertInto<Human<ToExtra>, dyn PlayerObj> for From
where
    From: PlayerObj + ?Sized,
    ToExtra: HumanExtra
{
    fn try_convert_into(self: Box<Self>) -> Box<Human<ToExtra>>
    {
        if self.as_any().is::<Human<ToExtra>>()
        {
            return self.upcast().downcast().unwrap()
        }
        Box::new(self.as_noextra().clone_with_extra())
    }
}
impl DowncastFrom<dyn PlayerObj> for dyn HumanObj
{
    fn is(from: &dyn PlayerObj) -> bool
    {
        from.is_human()
    }

    fn downcast_from_ref(from: &dyn PlayerObj) -> Option<&Self>
    {
        from.as_human()
    }

    fn downcast_from_mut(from: &mut dyn PlayerObj) -> Option<&mut Self>
    {
        from.as_human_mut()
    }

    fn downcast_from(from: Box<dyn PlayerObj>) -> Result<Box<Self>, Box<dyn PlayerObj>>
    {
        from.into_human()
    }
}