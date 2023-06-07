#![allow(dead_code)]

pub mod game; 
pub mod player;

use game::*;
use player::*;

#[cfg(test)]
mod test;

fn main() -> Result<(), SessionJoinError>
{
    let mut session = SessionTcp::new(None, 6666).unwrap();
    let mut ui = TUI;
    let me : Human = Human::new(&mut session, &mut ui)?;
    let you : Human = Human::new(&mut session, &mut ui)?;
    let players: [Box<dyn Player>; 2] = [Box::new(me), Box::new(you)];
    let mut rps_game : Game = Game::new(players, Box::new(session));

    rps_game.game_loop();

    Ok(())
}