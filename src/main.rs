#![allow(warnings)] // Disable all warnings

mod game;
mod game_debug;
mod models;
mod rng;
mod rules;

use game::Game;
use game_debug::debug_print;

fn main() {
  let game = Game::new(None);
  debug_print(&game);
}
