use super::*;

pub struct ActorClientFriend
{
    pub(super) send: Arc<RwLock<Option<ClientMessage>>>,
    pub(super) response: Arc<RwLock<Option<Result<ServerMessage, DeserializeTcpError>>>>,
    pub(super) awaiting_response: Arc<RwLock<bool>>
}

impl ActorClientFriend
{
    fn await_response_tcp_cycle(&self, stream: &mut TcpStream, buffer: &mut [u8]) -> Result<bool, TcpThreadError>
    {
        let len = stream.read(buffer)
            .map_err(|error| TcpThreadError::ReadStreamError(error))?;

        if len > 0
        {
            *self.awaiting_response.write()? = false;
            *self.response.write()? = Some(ServerMessage::try_from_tcp_message(buffer));
            return Ok(true);
        }
        Ok(false)
    }

    fn await_response_tcp(&self, stream: &mut TcpStream) -> Result<(), TcpThreadError>
    {
        let mut buffer = vec![];
        while !self.await_response_tcp_cycle(stream, &mut buffer)? {}
        Ok(())
    }

    fn stream_tcp_cycle(&self, stream: &mut TcpStream) -> Result<(), TcpThreadError>
    {
        if let Some(send) = self.send.write()?.take()
        {
            stream.write(&send.into_tcp_message())
                .map_err(|error| TcpThreadError::WriteStreamError(error))?;

            if *self.awaiting_response.read()?
            {
                self.await_response_tcp(stream)?
            }
        }
        Ok(())
    }

    pub fn stream_tcp(&self, mut stream: TcpStream) -> TcpThreadError
    {
        loop
        {
            if let Err(error) = self.stream_tcp_cycle(&mut stream)
            {
                return error
            }
        }
    }
}