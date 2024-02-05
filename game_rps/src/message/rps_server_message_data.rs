use serde::{Deserialize, Serialize};

use crate::{Choice, RpsEndState};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum RpsServerMessageData
{
    JoinResponse {
        id: u8
    },
    RoundOver {
        choices: [Choice; 2]
    },
    GameOver {
        end_state: RpsEndState
    }
}