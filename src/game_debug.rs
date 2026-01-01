use colored::Colorize;

use crate::card::CardColor;
use crate::game::Game;

/// Color a string based on card color
fn color_card_name(name: &str, color: &CardColor) -> colored::ColoredString {
  match color {
    CardColor::Blue => name.blue(),
    CardColor::Green => name.green(),
    CardColor::Red => name.red(),
    CardColor::Purple => name.magenta(),
  }
}

/// Print debug information about the game state
pub fn debug_print(game: &Game) {
  // Print seed
  println!(
    "{} {}",
    "Seed:".cyan().bold(),
    game.seed.to_string().yellow()
  );

  // Print current player and round
  println!(
    "{} {}",
    "Current Player:".cyan().bold(),
    game.current_player.to_string().yellow()
  );
  println!(
    "{} {}",
    "Current Round:".cyan().bold(),
    game.current_round.to_string().yellow()
  );

  // Print face-up cards for less than 7 deck
  let less_than_7_str: Vec<String> = game
    .less_than_7_face_up
    .iter()
    .map(|(card, count)| {
      format!(
        "({}, {})",
        color_card_name(card.name, &card.color),
        count.to_string().white()
      )
    })
    .collect();
  println!(
    "{} {}",
    "Less than 7:".blue().bold(),
    less_than_7_str.join(", ")
  );

  // Print face-up cards for greater than 6 deck
  let greater_than_6_str: Vec<String> = game
    .greater_than_6_face_up
    .iter()
    .map(|(card, count)| {
      format!(
        "({}, {})",
        color_card_name(card.name, &card.color),
        count.to_string().white()
      )
    })
    .collect();
  println!(
    "{} {}",
    "Greater than 6:".blue().bold(),
    greater_than_6_str.join(", ")
  );

  // Print face-up landmarks
  let landmark_str: Vec<String> = game
    .landmark_face_up
    .iter()
    .map(|landmark| landmark.name.yellow().to_string())
    .collect();
  println!("{} {}", "Landmarks:".blue().bold(), landmark_str.join(", "));

  // Print remaining cards in decks
  println!(
    "{} {} {}",
    "Remaining Less than 7 deck:".bright_black(),
    game.less_than_7_deck.len().to_string().white(),
    "cards".bright_black()
  );
  println!(
    "{} {} {}",
    "Remaining Greater than 6 deck:".bright_black(),
    game.greater_than_6_deck.len().to_string().white(),
    "cards".bright_black()
  );
  println!(
    "{} {} {}",
    "Remaining Landmark deck:".bright_black(),
    game.landmark_deck.len().to_string().white(),
    "cards".bright_black()
  );
}
