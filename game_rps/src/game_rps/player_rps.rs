pub mod human_rps;

use std::{ops::CoerceUnsized, marker::Unsize};

pub use human_rps::*;

use super::*;

#[derive(Debug)]
pub enum PlayerDecision
{
    Choose(Choice),
    Quit
}


pub trait PlayerRpsObj: PlayerObj + Upcast<dyn PlayerObj> + Is<dyn PlayerObj> + Unsize<dyn PlayerObj>
{
    fn make_decision(
        self: &mut Self,
        ui: &mut dyn UIRps,
        session: &dyn SessionRpsObj,
        choice_log: &[[Choice; 2]]
    ) -> Result<Option<PlayerDecision>, PlayerDecisionError>;
}
Object!(PlayerRpsObj);

impl<Medium> DyncastObj<dyn PlayerRpsObj, dyn PlayerObj> for Medium
where
    Medium: Is<dyn PlayerRpsObj> + Is<dyn PlayerObj> + ?Sized,
{
    type Obj = dyn PlayerRpsObj;
}
impl<Medium> DyncastObj<dyn PlayerObj, dyn PlayerRpsObj> for Medium
where
    Medium: Is<dyn PlayerRpsObj> + Is<dyn PlayerObj> + ?Sized,
{
    type Obj = dyn PlayerRpsObj;
}

impl TryConvertInto<dyn PlayerRpsObj, dyn PlayerObj> for dyn PlayerObj
{
    fn try_convert_into(self: Box<Self>) -> Result<Box<dyn PlayerRpsObj>, Box<dyn PlayerObj>>
    {
        self.try_convert_into().map(|human: Box<HumanRps>| human as Box<dyn PlayerRpsObj>)
    }
}
impl DowncastFromRef<dyn PlayerObj> for dyn PlayerRpsObj
{
    fn downcast_from_ref(from: &dyn PlayerObj) -> Option<&Self>
    {
        from.downcast_ref().map(|human: &HumanRps| human.upcast_ref())
    }
    fn downcast_from_mut(from: &mut dyn PlayerObj) -> Option<&mut Self>
    {
        from.downcast_mut().map(|human: &mut HumanRps| human.upcast_mut())
    }
}
impl DowncastFrom<dyn PlayerObj, dyn PlayerObj> for dyn PlayerRpsObj
{
    fn downcast_from(from: Box<dyn PlayerObj>) -> Result<Box<Self>, Box<dyn PlayerObj>>
    {
        from.downcast().map(|human: Box<HumanRps>| human.upcast())
    }
}

static_assertions::assert_impl_one!(dyn PlayerRpsObj: Dyncast<dyn PlayerObj>);
static_assertions::assert_impl_one!(dyn PlayerRpsObj: TryConvert<dyn PlayerObj>);