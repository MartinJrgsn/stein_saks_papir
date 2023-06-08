pub trait SerializeTcp
{
    fn into_tcp_message(&self) -> Vec<u8>;
}

impl SerializeTcp for std::time::SystemTimeError
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        let duration = self.duration();
        [duration.as_secs().to_le_bytes().to_vec(), duration.subsec_nanos().to_le_bytes().to_vec()].concat()
    }
}