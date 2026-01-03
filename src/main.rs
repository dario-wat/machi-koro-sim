#![allow(warnings)] // Disable all warnings

mod debug;
mod engine;
mod game;
mod models;
mod optimization;
mod player_strategies;
mod rng;
mod rules;

use std::collections::HashMap;

use rayon::prelude::*;

use crate::{
  debug::{debug_print_card_counts, debug_print_simulation_results},
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
    for (card, count) in result.card_counts.iter() {
      *total_card_counts.entry(*card).or_insert(0) += count;
    }
  }

  // Aggregate round statistics
  let total_rounds: usize = results.iter().map(|r| r.winning_round).sum();
  let min_round = results.iter().map(|r| r.winning_round).min().unwrap_or(0);
  let max_round = results.iter().map(|r| r.winning_round).max().unwrap_or(0);
  let avg_round = total_rounds as f64 / sim_count as f64;

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
  // println!();
  // println!("Winning Round Statistics:");
  // println!("  Average: {:.2} rounds", avg_round);
  // println!("  Minimum: {} rounds", min_round);
  // println!("  Maximum: {} rounds", max_round);
  // println!();
  // println!("Winning Round Distribution:");

  // let mut sorted_rounds: Vec<_> = round_distribution.iter().collect();
  // sorted_rounds.sort_by_key(|&(round, _)| round);

  // let max_count = *round_distribution.values().max().unwrap_or(&1);
  // let bar_width = 50;

  // for (round, count) in sorted_rounds {
  //   let bar_length = ((*count as f64 / max_count as f64) * bar_width as f64) as usize;
  //   let percentage = (*count as f64 / sim_count as f64) * 100.0;
  //   println!(
  //     "  Round {:2}: {:>5} ({:5.2}%) {}",
  //     round,
  //     count,
  //     percentage,
  //     "█".repeat(bar_length)
  //   );
  // }
  // println!();

  // println!("Winner Index Distribution:");

  // let mut sorted_winners: Vec<_> = winner_distribution.iter().collect();
  // sorted_winners.sort_by_key(|&(index, _)| index);

  // let max_winner_count = *winner_distribution.values().max().unwrap_or(&1);

  // for (index, count) in sorted_winners {
  //   let bar_length = ((*count as f64 / max_winner_count as f64) * bar_width as f64) as usize;
  //   let percentage = (*count as f64 / sim_count as f64) * 100.0;
  //   println!(
  //     "  Player {}: {:>5} ({:5.2}%) {}",
  //     index,
  //     count,
  //     percentage,
  //     "█".repeat(bar_length)
  //   );
  // }
  // println!();

  debug_print_simulation_results(total_card_counts);
}
