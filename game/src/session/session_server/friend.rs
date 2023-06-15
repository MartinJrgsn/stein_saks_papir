use std::{collections::HashMap, marker::PhantomData};

use super::*;

pub(super) struct SessionServerFriend<TransportType>
where
    TransportType: Transport + ?Sized
{
    pub(super) join: Arc<RwLock<Vec<Box<dyn PlayerObj>>>>,
    pub(super) join_response: Arc<RwLock<HashMap<Port, Result<(), JoinError>>>>,
    pub(super) send: Arc<RwLock<Vec<ServerMessage>>>,
    pub(super) phantom_data: PhantomData<TransportType>
}
impl<TransportType> SessionServerFriend<TransportType>
where
    TransportType: Transport + ?Sized
{
    fn handle_incoming_tcp(&self, stream: &mut TcpStream) -> Result<ServerResponse, TcpThreadError>
    {
        Ok(self.handle_message(ClientMessage::try_deserialize_tcp(&message)?, from_port)?)
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
        self.join.write()?.push(Box::new(Human::<()>::new(port, name.to_string())));

        loop
        {
            if let Some(result) = self.join_response.write()?.get(&port)
            {
                return Ok(result.map(|()| port))
            }
        }

        /*for player in self.players.write()?.iter_mut()
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
        Ok(Err(JoinError::GameFull))*/
    }
}