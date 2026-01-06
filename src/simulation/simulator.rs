use std::collections::HashMap;

use std::sync::Mutex;

use rayon::prelude::*;
use strum::IntoEnumIterator;

use crate::{
  engine::Engine,
  models::{player::OwnedCard, Card},
  player_strategies::{GreedyBestCardStrategy, RandomStrategy},
  simulation::accumulator::{SimulationAccumulator, SimulationResult},
};

pub struct GameResult {
  pub player_cards: Vec<Vec<OwnedCard>>,
  pub winner_index: usize,
  pub player_dice_rolls: Vec<Vec<(u8, u8)>>,
}

impl Engine {
  pub fn collect_data_for_simulation(&self) -> GameResult {
    GameResult {
      player_cards: self
        .game
        .players
        .iter()
        .map(|player| player.cards.clone())
        .collect(),
      winner_index: self.game.winner().expect("No winner found"),
      player_dice_rolls: self
        .game
        .players
        .iter()
        .map(|player| player.dice_rolls.clone())
        .collect(),
    }
  }
}

const NUM_PLAYERS: usize = 4;

pub struct Simulator;

impl Simulator {
  pub fn new() -> Self {
    Self {}
  }

  pub fn run(&self, sim_count: usize) -> SimulationResult {
    let mut accumulator = SimulationAccumulator::new();

    // Run simulations in parallel
    (0..sim_count).into_par_iter().for_each(|_| {
      let mut engine = Engine::new();

      engine.add_player_strategy(Box::new(RandomStrategy::new()));
      engine.add_player_strategy(Box::new(RandomStrategy::new()));
      engine.add_player_strategy(Box::new(RandomStrategy::new()));
      engine.add_player_strategy(Box::new(RandomStrategy::new()));
      // engine.add_player_strategy(Box::new(GreedyBestCardStrategy::new()));
      engine.run();

      let result = engine.collect_data_for_simulation();
      accumulator.accumulate(&result);
    });

    accumulator.finalize(sim_count, NUM_PLAYERS)
  }
}
