use std::{any::Any, fmt::Display};

use thiserror::Error;

#[derive(Debug, Error)]
pub struct JoinThreadError(pub Box<dyn Any + Send>);

impl Display for JoinThreadError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Unable to join thread: {:?}", self.0)
    }
}
