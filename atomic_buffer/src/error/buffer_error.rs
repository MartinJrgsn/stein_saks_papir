use poison_error_obj::{PoisonErrorUnguarded, PoisonErrorObj};

#[derive(Debug)]
pub enum BufferError
{
    NullPointerError,
    PoisonError(PoisonErrorUnguarded),
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