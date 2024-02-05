use super::*;

#[derive(Debug, Clone)]
pub enum GameRpsError
{
    PlayerDecisionError(PlayerDecisionError),
    InvalidPlayerError(usize)
}
impl From<PlayerDecisionError> for GameRpsError
{
    fn from(value: PlayerDecisionError) -> Self
    {
        Self::PlayerDecisionError(value)
    }
}