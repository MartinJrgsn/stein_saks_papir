use std::{error::Error, fmt::Display, hash::Hash};

use super::TransportObj;

pub trait Transport: TransportObj
{
    type Target: Display + Send + Sync + Clone + Copy + Eq + PartialEq + Hash;
    
    type MessageError: Send + Error;

    const NAME: &'static str;
    const LISTENER_STACK_SIZE: Option<usize> = None;
    const STREAM_STACK_SIZE: Option<usize> = None;
}