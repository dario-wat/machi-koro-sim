use std::collections::HashMap;

use std::sync::Mutex;

use rayon::prelude::*;
use strum::IntoEnumIterator;

use crate::{
  engine::Engine,
  models::{player::OwnedCard, Card},
  player_strategies::RandomStrategy,
  simulation::accumulator::{SimulationAccumulator, SimulationResult},
};

pub struct GameResult {
  pub player_cards: Vec<Vec<OwnedCard>>,
  pub winner_index: usize,
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
    }
  }
}

const NUM_PLAYERS: usize = 4;
const ROUND_BREAKDOWN: [usize; 3] = [5, 10, 15];

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
      for _ in 0..NUM_PLAYERS {
        engine.add_player_strategy(Box::new(RandomStrategy::new()));
      }
      engine.run();

      let result = engine.collect_data_for_simulation();
      accumulator.accumulate(&result);
    });

    accumulator.finalize(sim_count, NUM_PLAYERS)
  }
}
