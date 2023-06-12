pub mod human_rps;

use std::ops::CoerceUnsized;

pub use human_rps::*;

use super::*;

#[derive(Debug)]
pub enum PlayerDecision
{
    Choose(Choice),
    Quit
}

pub trait PlayerRps: PlayerRpsObj + TryConvert<dyn PlayerObj> + Upcast<dyn PlayerRpsObj> + DowncastFrom<dyn PlayerObj> + Is<dyn PlayerRpsObj>
{
    
}

pub trait PlayerRpsObj: Player
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
Is!(PlayerRpsObj);