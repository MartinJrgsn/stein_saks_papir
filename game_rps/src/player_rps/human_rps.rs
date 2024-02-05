pub mod human_rps_extra;

use game::player::Human;
pub use human_rps_extra::*;

use super::*;

pub type HumanRps = Human<HumanRpsExtra>;

impl PlayerRpsObj for HumanRps
{
    fn make_decision(
        self: &mut Self,
        ui: &mut dyn UIRps,
        session: &SessionRps,
        choice_log: &[[Choice; 2]]
    ) -> Result<Option<PlayerDecision>, PlayerDecisionError>
    {
        if !session.is_player_local(self)
        {
            Ok(self.get_extra_mut().received_decision.take())
        }
        else
        {
            Ok(ui.promt_for_choice(self, session, choice_log)?)
        }
    }
}