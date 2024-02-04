use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ClientMessage<ClientMessageData>
{
    Data(ClientMessageData),
    RequestLeave
}