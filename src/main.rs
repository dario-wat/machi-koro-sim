#![allow(warnings)] // Disable all warnings

mod card;
mod game;
mod game_debug;
mod landmark;
mod rng;

use game::Game;
use game_debug::debug_print;

fn main() {
  let game = Game::new(Some(12345));
  debug_print(&game);
}
