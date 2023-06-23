#[derive(Debug)]
pub enum RequestError<OnRequestError, SendError, ReceiveError>
{
    OnRequestError(OnRequestError),
    SendError(SendError),
    ReceiveError(ReceiveError)
}