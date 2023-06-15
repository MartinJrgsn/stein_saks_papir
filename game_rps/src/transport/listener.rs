use super::*;

impl<MessageType> Listen<TransportTcpUdp> for Listener<MessageType>
where
    MessageType: TryDeserializeTcp
{
    fn listen(&self, listener: TcpListener) -> <TransportTcpUdp as Transport>::ListenerError
    {
        for stream in listener.incoming()
        {
            if let Err(error) = self.on_incoming_tcp(stream)
            {
                return error;
            }
        }
        TcpThreadError::ListenerStopped
    }
}
impl<MessageType> Listener<MessageType>
where
    Self: Listen<TransportTcpUdp>,
    MessageType: TryDeserializeTcp
{
    fn on_incoming_tcp(&self, stream: Result<TcpStream, std::io::Error>)
        -> Result<(), <TransportTcpUdp as Transport>::ListenerError>
    {
        let mut stream = stream
            .map_err(|error| TcpThreadError::InvalidStream(error))?;

        let from_port = stream.local_addr()
            .map_err(|error| TcpThreadError::CannotRetrieveAddressFromStream(error))?
            .port();
        
        let mut message_bytes = vec![];
        stream.read(&mut message_bytes)
            .map_err(|error| TcpThreadError::ReadStreamError(error))?;

        let message = MessageType::try_deserialize_tcp(&message_bytes)?;

        self.receive_buffer.write()?.push((from_port, message));
        
        /*stream.write(&response.serialize_tcp())
            .map_err(|error| TcpThreadError::WriteStreamError(error))?;*/
    }
}