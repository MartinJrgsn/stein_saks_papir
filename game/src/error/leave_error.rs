use super::*;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[repr(u8)]
#[derive(Serialize, Deserialize, Error, Debug, Clone, Copy)]
pub enum LeaveError
{
    #[error("You have already left this session.")]
    AlreadyLeft
}