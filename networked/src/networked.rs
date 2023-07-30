use std::sync::Arc;

pub struct Networked<T>
{
    data: Arc<T>
}
impl<T> Networked<T>
{
    pub fn new(value: T) -> Self
    {
        Self {
            data: Arc::new(value)
        }
    }
}