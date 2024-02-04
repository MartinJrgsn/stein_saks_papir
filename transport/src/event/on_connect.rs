use crate::{transport::ListenerTransport, ParaStream, ReceiveBufferShare};

pub enum OnConnect<RequestType, ResponseType, TransportType>
where
    RequestType: Send + Sync,
    ResponseType: Send + Sync,
    TransportType: ListenerTransport<RequestType, ResponseType>
{
    NewConnection,
    DuplicateConnection(ParaStream<RequestType, ResponseType, TransportType, ReceiveBufferShare<RequestType, ResponseType, TransportType>>),
    ConnectError(TransportType::ConnectError),
}