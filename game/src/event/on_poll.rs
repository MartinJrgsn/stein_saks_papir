use serde::{Deserialize, Serialize};

use super::*;

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OnPoll<Target>
{
    OnJoin(OnJoin<Target>),
    OnLeave(OnLeave<Target>)
}