use std::fmt::Display;

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum RpsEndState
{
    PlayerQuit,
    GameOver
}

impl Display for RpsEndState
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::PlayerQuit => write!(f, "Player quit."),
            Self::GameOver => write!(f, "Game over."),
        }
    }
}