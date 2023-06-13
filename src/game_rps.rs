pub mod player_rps;
pub mod ui_rps;
pub mod error;

pub use player_rps::*;
pub use ui_rps::*;
pub use error::*;

use super::*;

pub enum GameRpsEndState
{
    PlayerQuit
}

pub struct GameRps
{
    choice: [Option<Choice>; 2],
    choice_log: Vec<[Choice; 2]>,
    session: Box<dyn Session<2>>
}

impl Game<2> for GameRps
{
    type GameEndResult = Result<GameRpsEndState, GameRpsError>;
    type UIType = dyn UIRps;

    fn new(session: Box<dyn Session<2>>) -> Self
    {
        Self {
            choice: [None; 2],
            choice_log: vec![],
            session
        }
    }

    fn game_loop(&mut self, ui: &mut Self::UIType) -> Self::GameEndResult
    {
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
                let (mut players, _) = actor.get_players_or_wait_mut()?;
                loop
                {
                    match self.choice
                    {
                        _ => {
                            for (player, choice) in players.iter_mut()
                                .zip(self.choice.iter_mut())
                            {
                                if choice.is_none()
                                {
                                    let human: &mut dyn HumanObj = <dyn HumanObj>::try_convert_get_mut(&mut *player)?;
                                    let player: &mut dyn PlayerRpsObj = <dyn PlayerRpsObj>::try_convert_get_mut(&mut *player)?;
                                    match player.make_decision(
                                        actor,
                                        &self.choice_log,
                                        ui
                                    )?
                                    {
                                        Some(decision) => match decision
                                        {
                                            PlayerDecision::Choose(player_choice) => *choice = Some(player_choice),
                                            PlayerDecision::Quit => return Ok(GameRpsEndState::PlayerQuit)
                                        }
                                        None => ()
                                    }
                                }
                            }
                        }
                        [Some(choice1), Some(choice2)] => {
                            let outcome = choice1.get_outcome(choice2);
                            println!("{}",if self.session.is_local(&**players[0]) {outcome} else {!outcome});
                            self.choice_log.push([choice1, choice2]);
                            self.choice = [None; 2];
                        },
                    }
                }
            }
        }
    }
}