use std::{time::Duration, sync::mpsc::SendError};

pub trait SendMessage<Message, Recipient>
{
    type SendError;

    #[doc(hidden)]
    fn is_ready(&self) -> Result<bool, SendError>;
    #[doc(hidden)]
    fn wait_until_ready(&self, timeout: Duration) -> Result<(), SendError>
    {
        Self::is_ready.try_repeat_until()
            .map_err(|error| SendError::Deadlock(error))
    }
    #[doc(hidden)]
    fn send_message_unchecked(&self, to: Recipient, message: Message) -> Result<(), SendError>;
    fn send_message(&mut self, to: Recipient, message: Message, timeout: Duration) -> Result<(), SendError>
    {
        self.wait_until_ready(timeout);
        self.send_message_unchecked(to, message, timeout)
    }
}
pub trait SendRequest<Request, Response, Recipient>:
    SendMessage<Request, Recipient>
    + ReceiveMessage<Response, Recipient>
{
    #[doc(hidden)]
    fn on_send_request(&mut self) -> Result<(), RequestError>
    {
        
    }
    fn send_request(&mut self, to: Recipient, request: Request, timeout: Duration) -> Result<Response, RequestError>
    {
        self.send_message(to, request, timeout);
        self.on_send_request();
        Ok(self.receive_or_wait(to, timeout)?)
    }
}