use std::collections::HashMap;

use crate::{engine::Engine, models::Card};

pub struct SimulationResult {
  pub winner_card_counts: HashMap<Card, usize>,
  pub winner_index: usize,
  pub winning_round: usize,
}

impl Engine {
  pub fn collect_data_for_simulation(&self) -> SimulationResult {
    let winner_index = self.game.winner().expect("No winner found");
    let winner = &self.game.players[winner_index];

    let mut winner_card_counts: HashMap<Card, usize> = HashMap::new();
    for card in &winner.cards {
      *winner_card_counts.entry(*card).or_insert(0) += 1;
    }

    SimulationResult {
      winner_card_counts,
      winner_index,
      winning_round: self.game.get_round(),
    }
  }
}
