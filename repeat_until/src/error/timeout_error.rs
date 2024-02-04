use std::{fmt::Display, time::Duration};

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub struct TimeoutError(pub Duration);
impl Display for TimeoutError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Timeout: {}s", self.0.as_secs_f64())
    }
}