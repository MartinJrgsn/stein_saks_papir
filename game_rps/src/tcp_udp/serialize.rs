pub trait SerializeTcp
{
    fn serialize_tcp(&self) -> Vec<u8>;
}

impl SerializeTcp for std::time::SystemTimeError
{
    fn serialize_tcp(&self) -> Vec<u8>
    {
        let duration = self.duration();
        [duration.as_secs().to_le_bytes().to_vec(), duration.subsec_nanos().to_le_bytes().to_vec()].concat()
    }
}