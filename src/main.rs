#![allow(warnings)] // Disable all warnings

mod card;
mod game;
mod game_debug;
mod landmark;
mod player;
mod rng;

use game::Game;
use game_debug::debug_print;

fn main() {
  let game = Game::new(None);
  debug_print(&game);
}
