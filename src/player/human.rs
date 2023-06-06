use crate::{game::*, player::PlayerDecisionError};
use std::{io};

use super::Player;

#[derive(Clone)] 
pub struct Human 
{
    name : String,
    memory : [Option<Choice>; Self::MEMORY_LENGTH],
}

impl Human 
{
    pub const MEMORY_LENGTH : usize = 5;

    pub fn new_player_with_name(name: String) -> Self
    {        
        Self
        { 
            name, 
            memory : [Option::None; Human::MEMORY_LENGTH],
        }
    }

    fn prompt_for_name() -> String
    {
        let mut input = String::new();
        println!("Enter player name:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    pub fn new_player(name : Option<String>) -> Self {
        Self::new_player_with_name(match name {
            None => Self::prompt_for_name(),
            Some(name) => name
        })
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_memory(&self) -> &[Option<Choice>; Self::MEMORY_LENGTH] {
        &self.memory
    }

    pub const fn get_memory_length(&self) -> usize {
        Human::MEMORY_LENGTH
    }

    // Not using append_memory anymore
    // pub fn append_memory(&mut self, choice : Choice) -> () {
    //     &self.memory.rotate_right(1);
    //     self.memory[0] = Some(choice);
    // }
}

impl Player for Human {
    fn is_human(self: &Self) -> bool
    {
        true
    }

    fn make_decision(self: &mut Self, player_names: [String; 2], choice_log: &[[Choice; 2]]) -> Result<Option<Choice>, PlayerDecisionError>
    {
        for mem in choice_log {
            println!(
                "Previous choices: \n{0}: {1}, {2}: {3}",
                player_names[0],
                mem[0],
                player_names[1],
                mem[1]
            );
        }

        let mut input = String::new();
        println!("{0} choose: Rock/Paper/Scissor:", self.get_name());
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let cin = input.trim();
        match cin 
        {
            "r" | "1" => Ok(Some(Choice::Rock)),
            "p" | "2" => Ok(Some(Choice::Paper)),
            "s" | "3" => Ok(Some(Choice::Scissor)),
            "q" => Err(PlayerDecisionError::Quit),
            _ => Ok(None),
        }            
    }

    fn get_name(self: &Self) -> &str {
        &self.name
    }
}