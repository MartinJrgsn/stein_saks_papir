use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::outcome::Outcome;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)] // clone explicit , copy implisit
#[repr(u8)]
pub enum Choice
{
    Paper = 0,
    Rock = 1,
    Scissor = 2,
}

impl Choice
{
    fn is_tie(self, opponents_choice : Choice) -> bool
    {
        self as u8 == opponents_choice as u8
    }

    fn is_win(self, opponents_choice : Choice) -> bool
    {
        (self as u8 + 1) % (core::mem::variant_count::<Self>() as u8) == opponents_choice as u8
    }

    pub fn get_outcome(self, opponents_choice : Choice) -> Outcome
    {
        if self.is_win(opponents_choice)
        {
            Outcome::Win
        }
        else if self.is_tie(opponents_choice)
        {
            Outcome::Draw
        }
        else
        {
            Outcome::Loss
        }
    }
}

impl Display for Choice 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self {
            Choice::Paper => write!(f, "Paper"),
            Choice::Rock => write!(f, "Rock"),
            Choice::Scissor => write!(f, "Scissor"),
        }
    }
}