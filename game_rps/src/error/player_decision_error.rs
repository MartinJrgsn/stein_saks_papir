use super::*;

use game::error::PromptError;
use poison_error_obj::PoisonErrorUnguarded;

#[derive(Debug, Clone)]
pub enum PlayerDecisionError
{
    PromptError(PromptError),
    PoisonError(PoisonErrorUnguarded)
}
impl From<PromptError> for PlayerDecisionError
{
    fn from(value: PromptError) -> Self
    {
        Self::PromptError(value)
    }
}
impl From<PoisonErrorUnguarded> for PlayerDecisionError
{
    fn from(value: PoisonErrorUnguarded) -> Self
    {
        Self::PoisonError(value)
    }
}