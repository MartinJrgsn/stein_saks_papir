use std::fmt::Display;

use poison_error_obj::{PoisonErrorUnguarded, PoisonErrorObj};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum BufferError
{
    NullPointerError,
    PoisonError(PoisonErrorUnguarded),
}
impl Display for BufferError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::NullPointerError => write!(f, "Buffer Error: Null Pointer Error"),
            Self::PoisonError(error) => write!(f, "Buffer Error: {}", error)
        }
    }
}
impl<'a, T> From<T> for BufferError
where
    T: PoisonErrorObj
{
    fn from(_: T) -> Self
    {
        Self::PoisonError(PoisonErrorUnguarded)
    }
}