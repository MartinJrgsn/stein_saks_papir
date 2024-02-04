use std::{ops::Try, time::Duration};

use crate::error::RequestError;

use super::*;

pub trait SendRequest<Request, Response, Recipient>:
    SendMessage<Request, Recipient>
    + ReceiveMessage<Response, Recipient>
where
    Recipient: Copy
{
    type OnRequestError;

    #[doc(hidden)]
    #[must_use]
    fn on_send_request(&mut self)
        -> Result<(), Self::OnRequestError>
    {
        Ok(())
    }
    #[must_use]
    fn send_request(&mut self, to: Recipient, request: Request, timeout: Duration)
        -> Result<Response, RequestError<Self::OnRequestError, Self::SendError, Self::ReceiveError>>
    where
        Result<bool, Self::SendError>: Try<Output = bool, Residual = Self::SendError>,
        Result<Option<Response>, Self::ReceiveError>: Try<Output = Option<Response>, Residual = Self::ReceiveError>
    {
        self.send_message(to, request, timeout)
            .map_err(|error| RequestError::SendError(error))?;
        self.on_send_request()
            .map_err(|error| RequestError::OnRequestError(error))?;
        self.receive_or_wait(to, timeout)
            .map_err(|error| RequestError::ReceiveError(error))
    }
}