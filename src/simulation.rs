use std::collections::HashMap;

use crate::{engine::Engine, models::Card};

pub struct GameResult {
  // Per player card counts at the end of the game
  pub card_counts: Vec<HashMap<Card, usize>>,
  pub winner_index: usize,
  pub winning_round: usize,
}

impl Engine {
  pub fn collect_data_for_simulation(&self) -> GameResult {
    let mut card_counts: Vec<HashMap<Card, usize>> = Vec::new();
    for player in &self.game.players {
      let mut player_card_counts: HashMap<Card, usize> = HashMap::new();
      for card in &player.cards {
        *player_card_counts.entry(*card).or_insert(0) += 1;
      }
      card_counts.push(player_card_counts);
    }

    GameResult {
      card_counts,
      winner_index: self.game.winner().expect("No winner found"),
      winning_round: self.game.get_round(),
    }
  }
}
