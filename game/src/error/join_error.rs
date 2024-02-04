use super::*;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[repr(u8)]
#[derive(Serialize, Deserialize, Error, Debug, Clone, Copy)]
pub enum JoinError
{
    #[error("The session is full. Please wait until players have left or the game is finished.")]
    GameFull {
        capacity: usize
    },
    #[error("You have already joined this session.")]
    AlreadyJoined
}