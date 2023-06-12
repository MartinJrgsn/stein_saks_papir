use super::*;

#[derive(Debug)]
pub enum PromtError
{
    StdInError(std::io::Error)
}