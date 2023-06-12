use super::*;

#[derive(Debug)]
pub enum PlayerDecisionError
{
    PromtError(PromtError),
    PoisonError(PoisonError)
}
impl From<PromtError> for PlayerDecisionError
{
    fn from(value: PromtError) -> Self
    {
        PlayerDecisionError::PromtError(value)
    }
}
impl From<PoisonError> for PlayerDecisionError
{
    fn from(value: PoisonError) -> Self
    {
        PlayerDecisionError::PoisonError(value)
    }
}