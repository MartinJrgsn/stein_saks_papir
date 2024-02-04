use serde::{Deserialize, Serialize};

use crate::error::JoinError;

use super::*;

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OnJoin<Target>
{
    Success {
        server_target: Target,
        client_target: Target
    },
    Failure(JoinError)
}