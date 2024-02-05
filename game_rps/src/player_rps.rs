pub mod human_rps;

use std::{ops::CoerceUnsized, marker::Unsize};

use clone_box::clone_box;
use game::player::{HumanObj, PlayerObj};
pub use human_rps::*;
use spellcast::{convert::*, downcast::*, impl_object, upcast::*, Is};

use self::error::PlayerDecisionError;

use super::*;

#[derive(Debug, Clone, Copy)]
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
        session: &SessionRps,
        choice_log: &[[Choice; 2]]
    ) -> Result<Option<PlayerDecision>, PlayerDecisionError>;
}
impl_object!(PlayerRpsObj);

impl TryConvertInto<dyn PlayerRpsObj, dyn PlayerObj> for dyn PlayerObj
{
    fn try_convert_into(self: Box<Self>) -> Result<Box<dyn PlayerRpsObj>, Box<dyn PlayerObj>>
    {
        self.into_human().map(|human| human.convert_into() as Box<dyn PlayerRpsObj>)
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

static_assertions::assert_impl_one!(dyn PlayerRpsObj: TryConvert<dyn PlayerObj>);