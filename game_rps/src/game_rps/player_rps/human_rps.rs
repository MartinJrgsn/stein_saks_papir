pub mod human_rps_extra;

pub use human_rps_extra::*;

use super::*;

pub type HumanRps = Human<HumanRpsExtra>;

impl PlayerRpsObj for HumanRps
{
    fn make_decision(
        self: &mut Self,
        ui: &mut dyn UIRps,
        session: &dyn SessionRpsObj,
        choice_log: &[[Choice; 2]]
    ) -> Result<Option<PlayerDecision>, PlayerDecisionError>
    {
        if !session.is_local(self)
        {
            Ok(self.get_extra_mut().received_decision.take())
        }
        else
        {
            Ok(ui.promt_for_choice(self, session, choice_log)?)
        }
    }
}