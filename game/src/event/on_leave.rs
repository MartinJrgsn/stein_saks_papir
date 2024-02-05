use serde::{Deserialize, Serialize};

use crate::error::LeaveError;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OnLeave<Target>
{
    Success {
        server_target: Target,
        client_target: Target,
        reason: LeaveReason
    },
    Failure(LeaveError)
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LeaveReason
{
    Kicked,
    Left
}