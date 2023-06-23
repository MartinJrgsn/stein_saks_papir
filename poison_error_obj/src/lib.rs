pub trait PoisonErrorObj: std::fmt::Debug + std::fmt::Display + std::error::Error
{

}
impl<T> PoisonErrorObj for std::sync::PoisonError<T>
{
    
}
impl<'a, T> From<std::sync::PoisonError<T>> for Box<dyn PoisonErrorObj + 'a>
where
    T: 'a
{
    fn from(error: std::sync::PoisonError<T>) -> Self
    {
        Box::new(error)
    }
}

#[derive(Debug)]
pub struct PoisonErrorUnguarded;
impl std::fmt::Display for PoisonErrorUnguarded
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Poison Error (guard dropped)")
    }
}
impl std::error::Error for PoisonErrorUnguarded {}
impl PoisonErrorObj for PoisonErrorUnguarded {}
impl From<PoisonErrorUnguarded> for Box<dyn PoisonErrorObj>
{
    fn from(error: PoisonErrorUnguarded) -> Self
    {
        Box::new(error)
    }
}
impl<T> From<std::sync::PoisonError<T>> for PoisonErrorUnguarded
{
    fn from(_: std::sync::PoisonError<T>) -> Self
    {
        Self
    }
}