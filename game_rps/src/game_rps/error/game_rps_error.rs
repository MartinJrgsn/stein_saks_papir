use super::*;

#[derive(Debug)]
pub enum GameRpsError
{
    PoisonError(PoisonError),
    PlayerDecisionError(PlayerDecisionError),
    InvalidPlayerError(usize)
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