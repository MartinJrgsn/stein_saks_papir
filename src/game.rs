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

pub trait Game<const PLAYER_COUNT: usize>
{
    type GameEndResult;
    type UIType: ?Sized;

    fn new(session: Box<dyn Session<PLAYER_COUNT>>) -> Self;
    fn game_loop(&mut self, ui: &mut Self::UIType) -> Self::GameEndResult;
}