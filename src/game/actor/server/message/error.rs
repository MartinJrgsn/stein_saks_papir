use super::*;

pub enum ServerMessageParseError
{
    InsufficientBufferLength(usize),
    NameParseError(std::string::FromUtf8Error),
    UnrecognizedHeader(u8)
}
impl From<std::string::FromUtf8Error> for ServerMessageParseError
{
    fn from(error: std::string::FromUtf8Error) -> Self
    {
        Self::NameParseError(error)
    }
}