use super::*;

pub enum ClientMessageParseError
{
    InsufficientBufferLength(usize),
    NameParseError(std::string::FromUtf8Error),
    UnrecognizedHeader(u8)
}
impl From<std::string::FromUtf8Error> for ClientMessageParseError
{
    fn from(error: std::string::FromUtf8Error) -> Self
    {
        Self::NameParseError(error)
    }
}