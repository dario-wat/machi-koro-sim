#![allow(warnings)] // Disable all warnings

mod debug;
mod engine;
mod game;
mod models;
mod optimization;
mod player_strategies;
mod rng;
mod rules;
mod simulation;

use std::collections::HashMap;

use rayon::prelude::*;

use crate::{
  debug::{
    debug_print_card_counts_stats, debug_print_round_statistics, debug_print_winner_distribution,
  },
  engine::Engine,
  models::Card,
  player_strategies::RandomStrategy,
};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let sim_count = if args.len() > 1 {
    args[1].parse::<usize>().unwrap_or_else(|_| {
      eprintln!("Invalid simulation count. Using default: 1");
      1
    })
  } else {
    1
  };

  println!("Running {} simulations in parallel...", sim_count);

  let start_time = std::time::Instant::now();

  let results = (0..sim_count)
    .into_par_iter()
    .map(|_| {
      let mut engine = Engine::new();
      engine.add_player_strategy(Box::new(RandomStrategy::new()));
      engine.add_player_strategy(Box::new(RandomStrategy::new()));
      engine.add_player_strategy(Box::new(RandomStrategy::new()));
      engine.add_player_strategy(Box::new(RandomStrategy::new()));
      engine.run();
      engine.collect_data_for_simulation()
    })
    .collect::<Vec<_>>();

  // Aggregate card counts
  let mut total_card_counts: HashMap<Card, usize> = HashMap::new();
  for result in &results {
    for (card, count) in result.winner_card_counts.iter() {
      *total_card_counts.entry(*card).or_insert(0) += count;
    }
  }

  // Round distribution
  let mut round_distribution: HashMap<usize, usize> = HashMap::new();
  for result in &results {
    *round_distribution.entry(result.winning_round).or_insert(0) += 1;
  }

  // Winner index distribution
  let mut winner_distribution: HashMap<usize, usize> = HashMap::new();
  for result in &results {
    *winner_distribution.entry(result.winner_index).or_insert(0) += 1;
  }

  let elapsed = start_time.elapsed();
  println!("Simulations completed in {:.2?}", elapsed);

  // Print round statistics and distributions
  debug_print_round_statistics(&round_distribution, sim_count);
  debug_print_winner_distribution(&winner_distribution, sim_count);

  println!();
  debug_print_card_counts_stats(total_card_counts);
}
