#![allow(warnings)] // Disable all warnings

mod debug;
mod engine;
mod game;
mod models;
mod player_strategies;
mod rng;
mod rules;

use std::collections::HashMap;

use crate::{
  debug::{debug_print_card_counts, debug_print_simulation_results},
  engine::Engine,
  models::Card,
  player_strategies::RandomStrategy,
};

fn main() {
  let mut total_card_counts: HashMap<Card, usize> = HashMap::new();

  let args: Vec<String> = std::env::args().collect();
  let sim_count = if args.len() > 1 {
    args[1].parse::<usize>().unwrap_or_else(|_| {
      eprintln!("Invalid simulation count. Using default: 1");
      1
    })
  } else {
    1
  };

  println!("Running {} simulations...", sim_count);

  let start_time = std::time::Instant::now();

  for _ in 0..sim_count {
    let mut engine = Engine::new();
    engine.add_player_strategy(Box::new(RandomStrategy::new()));
    engine.add_player_strategy(Box::new(RandomStrategy::new()));
    engine.add_player_strategy(Box::new(RandomStrategy::new()));
    engine.add_player_strategy(Box::new(RandomStrategy::new()));
    engine.run();

    // debug_print_card_counts(engine.collect_data_for_simulation());
    let card_counts = engine.collect_data_for_simulation();
    for (card, count) in card_counts.iter() {
      *total_card_counts.entry(*card).or_insert(0) += count;
    }
  }

  let elapsed = start_time.elapsed();
  println!("Simulations completed in {:.2?}", elapsed);

  debug_print_simulation_results(total_card_counts);
}
