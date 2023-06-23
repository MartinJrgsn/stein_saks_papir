use repeat_until::RepeatError;

pub struct DeadlockError(pub RepeatError);

impl From<RepeatError> for DeadlockError
{
    fn from(error: RepeatError) -> Self
    {
        Self(error)
    }
}