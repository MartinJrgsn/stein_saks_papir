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

    fn new(session: Box<dyn Session<PLAYER_COUNT>>);
    fn get_player_names(&self) -> [String; PLAYER_COUNT];
    fn game_loop(&mut self) -> Self::GameEndResult;
}