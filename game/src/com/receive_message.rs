use std::time::Duration;

pub trait ReceiveMessage<Message, Recipient>
{
    type ReceiveError;

    fn receive_once(&self, from: Recipient) -> Result<Option<Message>, Self::ReceiveError>;
    fn receive_or_wait(&self, from: Recipient, timeout: Duration) -> Result<Message, Self::ReceiveError>
    {
        Self::receive_once.try_repeat_until_some(timeout, (self, from))
    }
}