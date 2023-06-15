use super::*;

pub trait SendRequest<Request, Response, Recipient>:
    SendMessage<Request, Recipient>
    + ReceiveMessage<Response, Recipient>
{
    #[doc(hidden)]
    fn on_send_request(&mut self) -> Result<(), RequestError<Self::ReceiveError, Self::SendError>>
    {
        
    }
    fn send_request(&mut self, to: Recipient, request: Request, timeout: Duration) -> Result<Response, RequestError>
    {
        self.send_message(to, request, timeout);
        self.on_send_request();
        Ok(self.receive_or_wait(to, timeout)?)
    }
}