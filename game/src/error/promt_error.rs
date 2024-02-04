use thiserror::Error;

#[derive(Error, Debug)]
pub enum PromtError
{
    #[error("Text prompt failed.")]
    StdInError(std::io::Error)
}