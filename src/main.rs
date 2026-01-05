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
use std::sync::Mutex;

use rayon::prelude::*;
use strum::IntoEnumIterator;

use crate::{
  debug::debug_print_card_counts_stats, engine::Engine, models::Card,
  player_strategies::RandomStrategy, simulation::Simulator,
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

  let num_players = 4;

  println!("Running {} simulations in parallel...", sim_count);

  let start_time = std::time::Instant::now();

  let simulation = Simulator::new();
  let result = simulation.run(sim_count);

  let elapsed = start_time.elapsed();
  println!("Simulations completed in {:.2?}", elapsed);

  debug_print_card_counts_stats(
    result.total_card_counts,
    result.p_present_win,
    result.p_present_loss,
  );
}
