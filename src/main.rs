#![allow(dead_code)]

pub mod game; 

use game::*;
pub mod player;
use player::*;

#[cfg(test)]
mod test;

fn main() {
    let me : Human = Human::new_player(None);
    let you : Human = Human::new_player(None);
    let players: [Box<dyn Player>; 2] = [Box::new(me), Box::new(you)];
    let mut rps_game : Game = Game::new(players);

    rps_game.game_loop();
}