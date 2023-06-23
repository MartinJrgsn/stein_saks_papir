use std::time::Duration;

use repeat_until::{TryRepeatError, TryRepeatUntilBool};

use crate::error::DeadlockError;

pub trait SendMessage<Message, Recipient>
{
    type SendError: From<DeadlockError>;

    #[doc(hidden)]
    #[must_use]
    fn is_ready(&self) -> Result<bool, Self::SendError>;
    #[doc(hidden)]
    #[must_use]
    fn wait_until_ready(&self, timeout: Duration) -> Result<(), Self::SendError>
    {
        Self::is_ready.try_repeat_until_true((&self,), timeout)
            .map_err(|error| match error
            {
                TryRepeatError::FnError(error) => error,
                TryRepeatError::RepeatError(error) => DeadlockError::from(error).into()
            })
    }
    #[doc(hidden)]
    #[must_use]
    fn send_message_unchecked(&self, to: Recipient, message: Message) -> Result<(), Self::SendError>;
    #[must_use]
    fn send_message(&mut self, to: Recipient, message: Message, timeout: Duration) -> Result<(), Self::SendError>
    {
        self.wait_until_ready(timeout)?;
        self.send_message_unchecked(to, message)
    }
}