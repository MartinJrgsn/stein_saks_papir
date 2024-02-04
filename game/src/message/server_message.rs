use serde::{Deserialize, Serialize};

use crate::event::{OnJoin, OnLeave};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ServerMessage<ServerMessageData, Target, MessageError, GameResults>
{
    Data(ServerMessageData),
    MessageError(MessageError),
    OnJoin(OnJoin<Target>),
    OnLeave(OnLeave<Target>),
    OnEnd(GameResults)
}