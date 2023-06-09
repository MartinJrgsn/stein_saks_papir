pub mod player_rps;
pub mod error;

pub use player_rps::*;
pub use error::*;

use super::*;

pub struct GameRps
{
    choice: [Option<Choice>; 2],
    choice_log: Vec<[Choice; 2]>,
    players: [Box<dyn Player>; 2],
    session: Box<dyn Session<2>>
}

impl Game<2> for GameRps
{
    type GameEndResult = GameEndState;

    fn new(session: Box<dyn Session<2>>) -> Self {
        Self {
            choice: [None; 2],
            choice_log: vec![],
            players: session.wait_for_players(),
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

    fn game_loop(&mut self) -> Self::GameEndResult
    {
        let player_names: [String; 2] = self.get_player_names();
        match self.session.get_actor()
        {
            ActorAny::Client(actor) => {
                loop
                {
                    /*match actor.await_event()
                    {

                    }*/
                    todo!()
                }
            },
            ActorAny::Server(actor) => {
                loop
                {
                    match self.choice
                    {
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
                                        actor
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
    }
}

pub enum GameEndState
{
    PlayerQuit
}
impl From<PlayerDecisionError> for GameEndState
{
    fn from(error: PlayerDecisionError) -> Self
    {
        match error
        {
            PlayerDecisionError::Quit => Self::PlayerQuit
        }
    }
}