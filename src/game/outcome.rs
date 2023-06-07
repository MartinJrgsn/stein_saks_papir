use std::{fmt::Display, ops::{Not}};

#[derive(PartialEq, Debug)]
pub enum Outcome
{
    Win,
    Draw,
    Loss
}

impl Display for Outcome
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self {
            Self::Win  => write!(f, "You won! :-)"),
            Self::Draw => write!(f, "Draw :-|"),
            Self::Loss => write!(f, "You lost! :-("),
        }
    }
}

impl Not for Outcome
{
    type Output = Self;
    fn not(self) -> Self::Output
    {
        match self {
            Self::Win => Self::Loss,
            Self::Draw => Self::Draw,
            Self::Loss => Self::Win
        }
    }
}