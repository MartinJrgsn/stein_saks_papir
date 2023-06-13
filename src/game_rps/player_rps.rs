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
        actor: &ActorServer<2>,
        choice_log: &[[Choice; 2]],
        ui: &mut dyn UIRps
    ) -> Result<Option<PlayerDecision>, PlayerDecisionError>;
    fn upcast(self: &Self) -> &dyn PlayerObj;
    fn upcast_mut(self: &mut Self) -> &mut dyn PlayerObj;
}
Object!(PlayerRpsObj);

impl TryConvertInto<dyn PlayerRpsObj, dyn PlayerObj> for dyn PlayerObj
{
    fn try_convert_into(self: Box<Self>) -> Result<Box<dyn PlayerRpsObj>, Box<dyn PlayerObj>>
    {
        self.try_convert_into().map(|human: Box<HumanRps>| human as Box<dyn PlayerRpsObj>)
    }
}

static_assertions::assert_impl_one!(dyn PlayerRpsObj: TryConvert<dyn PlayerObj>);