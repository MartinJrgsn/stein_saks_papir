pub mod human_rps_extra;

pub use human_rps_extra::*;

use super::*;

pub type HumanRps = Human<HumanRpsExtra>;

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