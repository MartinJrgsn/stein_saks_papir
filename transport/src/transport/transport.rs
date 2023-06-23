use std::fmt::Display;

use super::TransportObj;

pub trait Transport: TransportObj
{
    type Id: Display + Send + Sync + Clone + Copy;
    type Target: Display + Copy;
    
    type DeserializeError: Send;

    const NAME: &'static str;
    const LISTENER_STACK_SIZE: Option<usize> = None;
    const STREAM_STACK_SIZE: Option<usize> = None;
}