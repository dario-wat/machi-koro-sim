mod card;
mod landmark;

use card::build_greater_than_6_deck;
use card::build_less_than_6_deck;
use landmark::build_landmark_deck;

fn main() {
  let less_than_6_deck = build_less_than_6_deck();
  println!("Less than 6 Deck ({} cards):", less_than_6_deck.len());
  for card in &less_than_6_deck {
    println!("{}", card);
  }

  let greater_than_6_deck = build_greater_than_6_deck();
  println!(
    "\nGreater than 6 Deck ({} cards):",
    greater_than_6_deck.len()
  );
  for card in &greater_than_6_deck {
    println!("{}", card);
  }

  let landmark_deck = build_landmark_deck();
  println!("\nLandmark Deck ({} landmarks):", landmark_deck.len());
  for landmark in &landmark_deck {
    println!("{}", landmark);
  }
}
