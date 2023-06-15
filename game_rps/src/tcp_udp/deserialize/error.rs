#[derive(Debug)]
pub enum DeserializeTcpError
{
    InsufficientBufferLength(usize),
    NameParseError(std::string::FromUtf8Error),
    ChoiceParseError,
    UnrecognizedHeader(u8),
    DataParseError
}
impl From<std::string::FromUtf8Error> for DeserializeTcpError
{
    fn from(error: std::string::FromUtf8Error) -> Self
    {
        Self::NameParseError(error)
    }
}