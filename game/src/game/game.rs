use crate::ui::UIObj;

use super::*;

pub trait Game<UIType>: GameObj + Sized
where
    UIType: UIObj + ?Sized
{
    type GameEndResult: Send + Sync + Clone + 'static;
    type Residual;
    
    fn on_quit(self: Self, ui: &mut UIType) -> Self::Residual;
    fn game_loop(mut self: Self, ui: &mut UIType) -> (Self::GameEndResult, Self::Residual)
    {
        loop
        {
            if let Some(end_result) = self.game_loop_once(ui)
            {
                return (end_result, self.on_quit(ui))
            }
        }
    }
    fn game_loop_once(&mut self, ui: &mut UIType) -> Option<Self::GameEndResult>;
}