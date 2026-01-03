#![allow(warnings)] // Disable all warnings

mod debug;
mod engine;
mod game;
mod models;
mod player_strategies;
mod rng;
mod rules;

use game::Game;

use crate::{engine::Engine, player_strategies::RandomStrategy};

fn main() {
  let mut engine = Engine::new();
  engine.add_player_strategy(Box::new(RandomStrategy::new()));
  engine.add_player_strategy(Box::new(RandomStrategy::new()));
  engine.run();
}
