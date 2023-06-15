use std::time::Duration;

use super::*;

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