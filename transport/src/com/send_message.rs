use std::{ops::Try, time::Duration};

use repeat_until::{RepeatError, TryRepeatError, TryRepeatUntilBool};

pub trait SendMessage<Message, Recipient>
{
    type SendError: From<RepeatError>;

    #[doc(hidden)]
    #[must_use]
    fn is_ready(&self) -> Result<bool, Self::SendError>;
    #[doc(hidden)]
    #[must_use]
    fn wait_until_ready(&self, timeout: Duration) -> Result<(), Self::SendError>
    where
        Result<bool, Self::SendError>: Try<Output = bool, Residual = Self::SendError>
    {
        Self::is_ready.try_repeat_until_true((&self,), timeout)
            .map_err(|error| match error
            {
                TryRepeatError::FnError(error) => error,
                TryRepeatError::RepeatError(error) => error.into()
            })
    }
    #[doc(hidden)]
    #[must_use]
    fn send_message_unchecked(&self, to: Recipient, message: Message) -> Result<(), Self::SendError>;
    #[must_use]
    fn send_message(&mut self, to: Recipient, message: Message, timeout: Duration) -> Result<(), Self::SendError>
    where
        Result<bool, Self::SendError>: Try<Output = bool, Residual = Self::SendError>
    {
        self.wait_until_ready(timeout)?;
        self.send_message_unchecked(to, message)
    }
}