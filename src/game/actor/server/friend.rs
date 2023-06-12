use super::*;

//dyn Player + Send + Sync
pub struct ActorServerFriend<const PLAYER_COUNT: usize>
{
    pub(super) players: Arc<RwLock<[Option<Box<dyn PlayerObj>>; PLAYER_COUNT]>>, // Use OrderedMap on nightly
    pub(super) send_queue: Arc<RwLock<Vec<ServerMessage>>>,
}
impl<const PLAYER_COUNT: usize> ActorServerFriend<PLAYER_COUNT>
{
    fn handle_incoming_tcp(&self, stream: &mut TcpStream) -> Result<ServerResponse, TcpThreadError>
    {
        let from_port = stream.local_addr()
            .map_err(|error| TcpThreadError::CannotRetrieveAddressFromStream(error))?
            .port();

        let mut message = vec![];
        stream.read(&mut message)
            .map_err(|error| TcpThreadError::ReadStreamError(error))?;

        Ok(self.handle_message(ClientMessage::try_from_tcp_message(&message)?, from_port)?)
    }

    fn on_incoming_tcp(&self, stream: Result<TcpStream, std::io::Error>) -> Result<(), TcpThreadError>
    {
        let mut stream = stream
            .map_err(|error| TcpThreadError::InvalidStream(error))?;

        // Will attempt to send error to client instead of panicking
        let response = ServerMessage::try_from(self.handle_incoming_tcp(&mut stream))?;
        
        stream.write(&response.into_tcp_message())
            .map_err(|error| TcpThreadError::WriteStreamError(error))?;

        Ok(())
    }
    pub fn listen_tcp(&self, listener: TcpListener) -> TcpThreadError
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

    fn handle_message(&self, message: ClientMessage, from_port: Port) -> Result<ServerResponse, TcpThreadError>
    {
        match message
        {
            ClientMessage::Name(name) => Ok(ServerResponse::OnJoin(
                match self.try_join_from_port(from_port, &name)
                    .map_err(|_| TcpThreadError::ThreadPoisoned)?
                {
                    Ok(port) => OnJoinEvent::Success(port),
                    Err(error) => OnJoinEvent::Failure(error)
                }
            )),
            _ => todo!()
        }
    }

    pub(super) fn try_join_from_port(&self, port: Port, name: &str)
        -> Result<Result<Port, JoinError>, PoisonError>
    {
        for player in self.players.write()?.iter_mut()
        {
            match player
            {
                None => {
                    *player = Some(Box::new(Human::<()>::new(port, name.to_string())));
                    return Ok(Ok(port))
                },
                Some(player) => if let Some(human) = player.as_human()
                {
                    if human.get_uid() == port
                    {
                        return Ok(Err(JoinError::AlreadyJoined))
                    }
                }
            }
        }
        Ok(Err(JoinError::GameFull))
    }
}