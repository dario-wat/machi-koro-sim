#![allow(warnings)] // Disable all warnings

mod engine;
mod game;
mod game_debug;
mod models;
mod player_strategies;
mod rng;
mod rules;

use game::Game;
use game_debug::debug_print;

use crate::{engine::Engine, player_strategies::RandomStrategy};

fn main() {
  let mut engine = Engine::new();
  engine.add_player_strategy(Box::new(RandomStrategy::new()));
  engine.add_player_strategy(Box::new(RandomStrategy::new()));
  engine.run();
}
