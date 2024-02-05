moddef::moddef!(
    flat(pub) mod {
        choice,
        end_state,
        rps_residual,
        outcome
    }
);

use std::sync::Mutex;

use game::{player::PlayerObj, game::{Game, GameObj}, message::{ClientMessage, ServerMessage}};
use transport_tcp::error::TcpMessageError;

use crate::error::GameRpsError;

use self::message::{RpsClientMessageData, RpsServerMessageData};

use spellcast::convert::*;

use super::*;

pub struct GameRps
{
    choice: [Option<Choice>; 2],
    residual: RpsResidual
}
impl GameRps
{
    pub fn new(session: SessionRps) -> Self
    {
        Self {
            choice: [None; 2],
            residual: RpsResidual {
                choice_log: vec![],
                session
            }
        }
    }
}

impl GameObj for GameRps
{
    fn player_capacity(&self) -> usize
    {
        2
    }
}
impl<UIType> Game<UIType> for GameRps
where
    UIType: UIRps
{
    type GameEndResult = Result<RpsEndState, GameRpsError>;
    type Residual = RpsResidual;
    
    fn game_loop_once(&mut self, ui: &mut UIType) -> Option<Self::GameEndResult>
    {
        self.residual.session.capacity = self.player_capacity();
        let choice = Arc::new(Mutex::new(self.choice.clone()));
        let choice_ = choice.clone(); 
        let players: Vec<_> = self.residual.session.await_players(
            |human, data| {
                match data
                {
                    RpsClientMessageData::Choice(choice) => {
                        if let Some(human) = HumanRps::try_convert_get_mut(human)
                        {
                            human.get_extra_mut().received_decision = Some(PlayerDecision::Choose(choice));
                        }
                    },
                }
            },
            move |human, data| {
                match data
                {
                    RpsServerMessageData::JoinResponse { id } => todo!(),
                    RpsServerMessageData::RoundOver { choices } => {
                        *choice_.lock().unwrap() = choices.map(|choice| Some(choice));
                    },
                    RpsServerMessageData::GameOver { end_state } => todo!(),
                }
            },
            |human| {
                if let Some(human) = HumanRps::try_convert_get_mut(human)
                {
                    println!("{} joined!", human.get_name());
                }
            }
        ).expect("Error awaiting players.")
            .into_iter()
            .enumerate()
            .map(|(n, _)| n)
            .collect();
        self.choice = choice.lock().unwrap().clone();
        match self.choice
        {
            [Some(choice1), Some(choice2)] => {
                let outcome = choice1.get_outcome(choice2);
                println!("{}", if self.residual.session.is_player_local(&*self.residual.session.get_players()[0]) {outcome} else {!outcome});
                self.residual.choice_log.push([choice1, choice2]);
                self.choice = [None; 2];
            },
            _ => {
                for (i, (n, choice)) in players.into_iter()
                    .zip(self.choice.iter_mut())
                    .enumerate()
                {
                    if choice.is_none()
                    {
                        let mut player = self.residual.session.get_players()[n].clone();
                        match <dyn PlayerRpsObj>::try_convert_get_mut(&mut player)
                        {
                            Some(player) => {
                                match player.make_decision(
                                    ui,
                                    &self.residual.session,
                                    &self.residual.choice_log
                                ).expect("Invalid decision.")
                                {
                                    Some(decision) => match decision
                                    {
                                        PlayerDecision::Choose(player_choice) => *choice = Some(player_choice),
                                        PlayerDecision::Quit => return Some(Ok(RpsEndState::PlayerQuit))
                                    }
                                    None => ()
                                }
                            },
                            None => {
                                return Some(Err(GameRpsError::InvalidPlayerError(i)))
                            }
                        }
                        self.residual.session.get_players_mut()[n] = player;
                    }
                }
            }
        }
        None
    }

    fn on_quit(self: Self, ui: &mut UIType) -> RpsResidual
    {
        ui.on_quit();
        self.residual
    }
}