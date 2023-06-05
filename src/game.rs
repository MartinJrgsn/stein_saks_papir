// use std::mem;
// use std::{io};

use std::{io, fmt::format};

use crate::player::*;
pub mod choice;
pub use crate::choice::*;

pub struct Game {
    choice : [Option<Choice>; 2],
    choice_log : Vec<[Choice; 2]>,
    players : [Box<dyn Player>; 2]
}

impl Game {
    pub fn new(players : [Box <dyn Player>; 2]) -> Self {
        Self {
            choice : [None; 2],
            choice_log : vec![],
            players,
        }
    }

    fn outcome_to_string(&self, wld : Option<bool>) -> String {
        match wld {
            Some(true)  =>  format!("{0} wins!", self.players[0].get_name()),
            None        =>  format!("Draw :-|"),
            Some(false) =>  format!("{0} looses!", self.players[0].get_name()),
        }
    }

    fn get_player_names(&self) -> [String; 2]
    {
        [self.players[0].get_name().to_string(), self.players[1].get_name().to_string()]
    }

    pub fn game_loop(&mut self) -> GameEndState
    {
        let player_names: [String; 2] = self.get_player_names();
        loop {
            match self.choice {
                [Some(player1), Some(player2)] => {
                    println!("{}", self.outcome_to_string(player1.get_outcome(player2)));
                    self.choice_log.push([player1, player2]);
                    self.choice = [None; 2];
                },
                _ => {
                    for (player, choice) in self.players.iter_mut()
                        .zip(self.choice.iter_mut())
                    {
                        if choice.is_none()
                        {
                            match player.make_decision(
                                player_names.clone(),
                                &self.choice_log
                            )
                            {
                                Ok(player_choice) => *choice = player_choice,
                                Err(error) => return error.into()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub enum GameEndState
{
    PlayerDecisionError(PlayerDecisionError)
}

impl Into<GameEndState> for PlayerDecisionError
{
    fn into(self) -> GameEndState
    {
        GameEndState::PlayerDecisionError(self)
    }
}