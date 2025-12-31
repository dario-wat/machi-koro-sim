#![allow(warnings)] // Disable all warnings

mod card;
mod landmark;
mod rng;

use card::build_greater_than_6_deck;
use card::build_less_than_6_deck;
use landmark::build_landmark_deck;
use rng::Rng;

fn main() {
  // Test RNG with seed
  //   println!("=== Testing RNG with seed 12345 ===");
  let mut seeded_rng = Rng::new_with_seed(12345);
  //   println!("Rolling single die 5 times:");
  //   for i in 1..=5 {
  //     let roll = seeded_rng.roll_die();
  //     println!("  Roll {}: {}", i, roll);
  //   }
  //   println!("Rolling two dice 3 times:");
  //   for i in 1..=3 {
  //     let roll = seeded_rng.roll_two_dice();
  //     println!("  Roll {}: {}", i, roll);
  //   }

  //   // Test RNG without seed (random)
  //   println!("\n=== Testing RNG without seed (random) ===");
  let mut random_rng = Rng::new();
  //   for i in 1..=10 {
  //     let roll = random_rng.roll_die();
  //     println!("  Roll {}: {}", i, roll);
  //   }
  //   for i in 1..=10 {
  //     let roll = random_rng.roll_two_dice();
  //     println!("  Roll {}: {}", i, roll);
  //   }

  let mut less_than_6_deck = build_less_than_6_deck();
  random_rng.shuffle(&mut less_than_6_deck);
  println!("\nLess than 6 Deck ({} cards):", less_than_6_deck.len());
  for card in &less_than_6_deck {
    println!("{}", card.name);
  }

  //   let greater_than_6_deck = build_greater_than_6_deck();
  //   println!(
  //     "\nGreater than 6 Deck ({} cards):",
  //     greater_than_6_deck.len()
  //   );
  //   for card in &greater_than_6_deck {
  //     println!("{}", card);
  //   }

  //   let landmark_deck = build_landmark_deck();
  //   println!("\nLandmark Deck ({} landmarks):", landmark_deck.len());
  //   for landmark in &landmark_deck {
  //     println!("{}", landmark);
  //   }
}
