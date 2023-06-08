use crate::{game::*, player::PlayerDecisionError};
use std::{io};

use super::Player;

#[derive(Clone)] 
pub struct Human 
{
    uid: Port,
    name: String
}

impl Human 
{
    pub const MEMORY_LENGTH : usize = 5;

    pub fn new(session: &mut dyn Session, ui: &mut dyn UI) -> Result<Self, RequestJoinError>
    {
        let (uid, name) = session.try_join(ui)?;
        Ok(Self
        {
            uid,
            name
        })
    }

    pub fn get_uid(&self) -> Port
    {
        self.uid
    }
}

impl Player for Human {
    fn as_human(self: &Self) -> Option<&Human>
    {
        Some(self)
    }
    fn as_human_mut(self: &mut Self) -> Option<&mut Human>
    {
        Some(self)
    }

    fn make_decision(self: &mut Self, player_names: [String; 2], choice_log: &[[Choice; 2]], session: &mut dyn Session) -> Result<Option<Choice>, PlayerDecisionError>
    {
        session.player_make_decision(self.uid)
        /*for mem in choice_log {
            println!(
                "Previous choices: \n{0}: {1}, {2}: {3}",
                player_names[0],
                mem[0],
                player_names[1],
                mem[1]
            );
        }

        let mut input = String::new();
        println!("{0} choose: Rock/Paper/Scissor:", self.get_name(actor));
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let cin = input.trim();
        match cin 
        {
            "r" | "1" => Ok(Some(Choice::Rock)),
            "p" | "2" => Ok(Some(Choice::Paper)),
            "s" | "3" => Ok(Some(Choice::Scissor)),
            "q" => Err(PlayerDecisionError::Quit),
            _ => Ok(None),
        }*/
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}