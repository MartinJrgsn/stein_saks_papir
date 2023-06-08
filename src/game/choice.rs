use std::{fmt::Display, num::TryFromIntError};

use super::{outcome::Outcome, TryDeserializeTcp, DeserializeTcpError, SerializeTcp};

#[derive(Clone, Copy, Debug)] // clone explicit , copy implisit
#[repr(u8)]
pub enum Choice
{
    Paper = 0,
    Rock = 1,
    Scissor = 2,
}

impl Choice
{
    fn is_tie(self, oponents_choice : Choice) -> bool
    {
        return self as u8 == oponents_choice as u8//;
    }

    fn is_win(self, oponents_choice : Choice) -> bool
    {
        return (self as u8 + 1) % (Self::LENGTH as u8) == oponents_choice as u8//;
    }

    pub fn get_outcome(self, oponents_choice : Choice) -> Outcome
    {
        if self.is_win(oponents_choice)
        {
            Outcome::Win
        }
        else if self.is_tie(oponents_choice)
        {
            Outcome::Draw
        }
        else
        {
            Outcome::Loss
        }
    }
    
    pub const LENGTH : usize = 3;
    const VALUES : [Choice; Self::LENGTH] = [Choice::Paper, Choice::Rock, Choice::Scissor];
}

impl TryFrom<u8> for Choice
{
    type Error = ();
    fn try_from(value : u8) -> Result<Choice, Self::Error>
    {
        if value >= Choice::LENGTH as u8
        {
            return Err(())
        }
        
        Ok(Choice::VALUES[value as usize])
    }
}
impl TryDeserializeTcp for Choice
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        Self::try_from(
            *bytes.get(0)
                .ok_or_else(|| DeserializeTcpError::InsufficientBufferLength(bytes.len()))?
        ).map_err(|_| DeserializeTcpError::ChoiceParseError)
    }
}
impl SerializeTcp for Choice
{
    fn into_tcp_message(&self) -> Vec<u8>
    {
        vec![*self as u8]
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