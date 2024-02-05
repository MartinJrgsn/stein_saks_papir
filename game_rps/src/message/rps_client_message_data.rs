use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::Choice;

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum RpsClientMessageData
{
    Choice(Choice)
}