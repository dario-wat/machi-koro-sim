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

  let num_players = 4;

  println!("Running {} simulations in parallel...", sim_count);

  let start_time = std::time::Instant::now();

  // Initialize tracking hashmaps
  let total_card_counts = Mutex::new(HashMap::new());
  let win_present_card_counts = Mutex::new(HashMap::new());
  let loss_present_card_counts = Mutex::new(HashMap::new());

  // Run simulations in parallel
  (0..sim_count).into_par_iter().for_each(|_| {
    let mut engine = Engine::new();
    for _ in 0..num_players {
      engine.add_player_strategy(Box::new(RandomStrategy::new()));
    }
    engine.run();

    let result = engine.collect_data_for_simulation();

    // Update total card counts
    let mut total = total_card_counts.lock().unwrap();
    for (card, count) in result.card_counts[result.winner_index].iter() {
      *total.entry(*card).or_insert(0) += count;
    }

    // Update card presence counts
    let mut presence = win_present_card_counts.lock().unwrap();
    for card in result.card_counts[result.winner_index].keys() {
      *presence.entry(*card).or_insert(0) += 1;
    }

    // Update loss card presence counts
    let mut loss_presence = loss_present_card_counts.lock().unwrap();
    for player_index in 0..result.card_counts.len() {
      if player_index == result.winner_index {
        continue;
      }
      for card in result.card_counts[player_index].keys() {
        *loss_presence.entry(*card).or_insert(0) += 1;
      }
    }
  });

  // Extract from Mutex
  let total_card_counts = total_card_counts.into_inner().unwrap();

  let win_present_card_counts = win_present_card_counts.into_inner().unwrap();
  let mut p_present_win: HashMap<Card, f64> = HashMap::new();
  for card in Card::iter() {
    p_present_win.insert(
      card,
      *win_present_card_counts.get(&card).unwrap_or(&0) as f64 / sim_count as f64,
    );
  }

  let loss_present_card_counts = loss_present_card_counts.into_inner().unwrap();
  let mut p_present_loss: HashMap<Card, f64> = HashMap::new();
  for card in Card::iter() {
    p_present_loss.insert(
      card,
      *loss_present_card_counts.get(&card).unwrap_or(&0) as f64
        / (sim_count * (num_players - 1)) as f64,
    );
  }

  let elapsed = start_time.elapsed();
  println!("Simulations completed in {:.2?}", elapsed);

  debug_print_card_counts_stats(total_card_counts, p_present_win, p_present_loss);
}
