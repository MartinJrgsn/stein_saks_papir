pub auto trait Unsized {}

impl<T> !Unsized for T
where
    T: Sized
{
    
}