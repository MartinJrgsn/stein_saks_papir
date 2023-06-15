pub struct Listener<MessageType>
{
    receive_buffer: Arc<RwLock<Vec<(Port, MessageType)>>>
}
impl<MessageType> Next for Listener<MessageType>
{
    
}