use repeat_until::RepeatError;

pub struct TimeoutError(pub RepeatError);

impl From<RepeatError> for TimeoutError
{
    fn from(error: RepeatError) -> Self
    {
        Self(error)
    }
}