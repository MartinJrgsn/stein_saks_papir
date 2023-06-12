use crate::game::PoisonError;

use super::player_decision_error::PlayerDecisionError;

#[derive(Debug)]
pub enum GameRpsError
{
    PoisonError(PoisonError),
    PlayerDecisionError(PlayerDecisionError)
}
impl From<PoisonError> for GameRpsError
{
    fn from(error: PoisonError) -> Self
    {
        Self::PoisonError(error)
    }
}
impl From<PlayerDecisionError> for GameRpsError
{
    fn from(value: PlayerDecisionError) -> Self
    {
        Self::PlayerDecisionError(value)
    }
}