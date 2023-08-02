use crate::{transport::{ListenerTransport}, ParaStream, ReceiveBufferShare};

pub enum OnConnect<MessageType, TransportType>
where
    MessageType: Send + Sync,
    TransportType: ListenerTransport<MessageType>
{
    NewConnection,
    DuplicateConnection(ParaStream<MessageType, TransportType, ReceiveBufferShare<MessageType, TransportType>>),
    ConnectError(TransportType::ConnectError),
}