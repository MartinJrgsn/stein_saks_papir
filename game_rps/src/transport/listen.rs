pub trait Listen<TransportType>
where
    TransportType: Transport
{
    fn listen(&self, listener: TransportType::ListenerType) -> <TransportType as Transport>::ListenerError;
}