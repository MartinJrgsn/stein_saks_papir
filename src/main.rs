#![allow(dead_code)]

pub mod game; // #include "game.rs"
use game::*; // == using game

#[cfg(test)]
mod test;


fn main() {
    let rps_game : Game = Game::new();
}