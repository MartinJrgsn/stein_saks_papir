pub mod choice;
pub mod session;
pub mod outcome;
pub mod ui;

use std::time::Duration;

pub use choice::*;
pub use session::*;
pub use outcome::*;
pub use ui::*;

use crate::repeat_until::RepeatUntilSome;

use super::*;

pub trait GameObj<SessionType, UIType>
where
    SessionType: SessionObj + ?Sized,
    UIType: UI + ?Sized
{
    fn get_session(self: &Self) -> &SessionType;
    fn get_session_mut(self: &mut Self) -> &mut SessionType;
}

pub trait Game<SessionType, UIType>: GameObj<SessionType, UIType> + Sized
where
    SessionType: SessionObj + ?Sized,
    UIType: UI + ?Sized
{
    type GameEndResult;

    fn new(session: Box<SessionType>) -> Self;
    fn quit(self: Self, ui: &mut UIType) -> Box<SessionType>;
    fn game_loop(self: Self, ui: &mut UIType) -> (Self::GameEndResult, Box<SessionType>)
    {
        loop
        {
            if let Some(end_result) = self.game_loop_once(ui)
            {
                return (end_result, self.quit(ui))
            }
        }
    }
    fn game_loop_once(&mut self, ui: &mut UIType) -> Option<Self::GameEndResult>;
}