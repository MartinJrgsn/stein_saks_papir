pub mod choice;
pub mod actor;
pub mod session;
pub mod outcome;
pub mod ui;

pub use choice::*;
pub use actor::*;
pub use session::*;
pub use outcome::*;
pub use ui::*;

use super::*;

pub struct Game {
    choice : [Option<Choice>; 2],
    choice_log : Vec<[Choice; 2]>,
    players : [Box<dyn Player>; 2],
    session: Box<dyn Session>
}

impl Game {
    pub fn new(players : [Box <dyn Player>; 2], session: Box<dyn Session>) -> Self {
        Self {
            choice : [None; 2],
            choice_log : vec![],
            players,
            session
        }
    }

    fn get_player_names(&self) -> [String; 2]
    {
        [
            self.players[0].get_name().to_string(),
            self.players[1].get_name().to_string()
        ]
    }

    pub fn game_loop(&mut self) -> GameEndState
    {
        let player_names: [String; 2] = self.get_player_names();
        loop {
            match self.choice {
                [Some(player1), Some(player2)] => {
                    let outcome = player1.get_outcome(player2);
                    println!("{}", if self.session.is_user(&*self.players[0]) {outcome} else {!outcome});
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
                                &self.choice_log,
                                &mut *self.session
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