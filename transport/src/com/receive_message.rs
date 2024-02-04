use std::{ops::Try, time::Duration};

use repeat_until::{RepeatError, TimeoutError, TryRepeatError, TryRepeatUntilSome};

pub trait ReceiveMessage<Message, Recipient>
where
    Recipient: Copy
{
    type ReceiveError: From<RepeatError>;

    fn receive_once(&self, from: Recipient) -> Result<Option<Message>, Self::ReceiveError>;
    fn receive_or_wait(&self, from: Recipient, timeout: Duration) -> Result<Message, Self::ReceiveError>
    where   
        Result<Option<Message>, Self::ReceiveError>: Try<Output = Option<Message>, Residual = Self::ReceiveError>
    {
        Self::receive_once.try_repeat_until_some((self, from), timeout)
            .map_err(|error| match error
            {
                TryRepeatError::RepeatError(error) => error.into(),
                TryRepeatError::FnError(error) => error,
            })
    }
}