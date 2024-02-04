use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct OnLeave<Target>
{
    pub server_target: Target,
    pub client_target: Target,
    pub reason: LeaveReason
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LeaveReason
{
    Kicked,
    Left
}