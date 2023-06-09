use super::*;

pub enum PlayerDecisionError
{
    PoisonError
}
impl From<PoisonError> for PlayerDecisionError
{
    fn from(value: PoisonError) -> Self
    {
        PlayerDecisionError::PoisonError
    }
}