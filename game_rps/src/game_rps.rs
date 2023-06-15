pub mod player_rps;
pub mod ui_rps;
pub mod session_rps;
pub mod error;

pub use player_rps::*;
pub use ui_rps::*;
pub use session_rps::*;
pub use error::*;

use super::*;

pub enum GameRpsEndState
{
    PlayerQuit
}

pub struct GameRps<SessionType>
where
    SessionType: SessionRpsObj + ?Sized
{
    choice: [Option<Choice>; 2],
    choice_log: Vec<[Choice; 2]>,
    session: Box<SessionType>
}

impl<SessionType, UIType> GameObj<SessionType, UIType> for GameRps<SessionType>
where
    SessionType: SessionRpsObj + ?Sized,
    UIType: UIRps + ?Sized
{
    fn get_session(self: &Self) -> &SessionType
    {
        &self.session
    }

    fn get_session_mut(self: &mut Self) -> &mut SessionType
    {
        &mut self.session
    }
}
default impl<SessionType, UIType> Game<SessionType, UIType> for GameRps<SessionType>
where
    SessionType: SessionRpsObj + ?Sized,
    UIType: UIRps + ?Sized
{
    type GameEndResult = Result<GameRpsEndState, GameRpsError>;

    fn new(session: Box<SessionType>) -> Self
    {
        Self {
            choice: [None; 2],
            choice_log: vec![],
            session
        }
    }
    
    fn game_loop_once(&mut self, ui: &mut UIType) -> Option<Self::GameEndResult>
    {
        /*match actor.await_event()
        {

        }*/
        todo!()
    }

    fn quit(self: Self, ui: &mut UIType) -> Box<SessionType>
    {
        ui.on_quit();
        self.session
    }
}
impl<SessionType, UIType> Game<SessionType, UIType> for GameRps<SessionType>
where
    SessionType: SessionRpsObj + SessionServerObj + ?Sized,
    UIType: UIRps + ?Sized
{
    fn game_loop_once(&mut self, ui: &mut UIType) -> Option<Self::GameEndResult>
    {
        let (mut players, _) = self.session.get_players_or_wait_mut()?;
        match self.choice
        {
            _ => {
                for (i, (player, choice)) in players.iter_mut()
                    .zip(self.choice.iter_mut())
                    .enumerate()
                {
                    if choice.is_none()
                    {
                        match <dyn PlayerRpsObj>::try_convert_get_mut(&mut *player)
                        {
                            Some(player) => {
                                match player.make_decision(
                                    self.session.get_actor(),
                                    &self.choice_log,
                                    ui
                                )?
                                {
                                    Some(decision) => match decision
                                    {
                                        PlayerDecision::Choose(player_choice) => *choice = Some(player_choice),
                                        PlayerDecision::Quit => return Some(Ok(GameRpsEndState::PlayerQuit))
                                    }
                                    None => ()
                                }
                            },
                            None => {
                                return Some(Err(GameRpsError::InvalidPlayerError(i)))
                            }
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
        None
    }
}