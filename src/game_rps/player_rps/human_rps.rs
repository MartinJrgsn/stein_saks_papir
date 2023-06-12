pub mod human_rps_extra;

pub use human_rps_extra::*;

use super::*;

pub type HumanRps = Human<HumanRpsExtra>;

impl Player for HumanRps
{
    
}
impl PlayerRps for HumanRps
{
    
}

impl PlayerRpsObj for HumanRps
{
    fn make_decision(
        self: &mut Self,
        actor: &ActorServer<2>,
        choice_log: &[[Choice; 2]],
        ui: &mut dyn UIRps
    ) -> Result<Option<PlayerDecision>, PlayerDecisionError>
    {
        if !actor.is_local(self)
        {
            Ok(self.get_extra_mut().received_decision.take())
        }
        else
        {
            Ok(ui.promt_for_choice(self, actor, choice_log)?)
        }
    }
    fn upcast(self: &Self) -> &dyn PlayerObj
    {
        self
    }
    fn upcast_mut(self: &mut Self) -> &mut dyn PlayerObj
    {
        self
    }
}
impl<From> TryConvertInto<dyn PlayerRpsObj, dyn PlayerObj> for From
where
    From: PlayerObj + ?Sized,
    dyn PlayerRpsObj: PlayerObj
{
    fn try_convert_into(self: Box<Self>) -> Result<Box<dyn PlayerRpsObj>, Box<dyn Player>>
    {
        self.into_human().map(|human| HumanRps::convert_from(human))
    }
}