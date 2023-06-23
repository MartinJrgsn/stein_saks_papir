use std::time::Duration;

use repeat_until::TryRepeatUntilSome;

use crate::error::TimeoutError;

pub trait ReceiveMessage<Message, Recipient>
where
    Recipient: Copy
{
    type ReceiveError: From<TimeoutError>;

    fn receive_once(&self, from: Recipient) -> Result<Option<Message>, Self::ReceiveError>;
    fn receive_or_wait(&self, from: Recipient, timeout: Duration) -> Result<Message, Self::ReceiveError>
    {
        Self::receive_once.try_repeat_until_some((self, from), timeout)
            .map_err(|error| error.flat_map(|error| TimeoutError::from(error).into()))
    }
}