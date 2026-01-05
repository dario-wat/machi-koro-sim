use crate::models::player::OwnedCard;
use crate::models::Card;
use crate::simulation::simulator::GameResult;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use strum::IntoEnumIterator;

const ROUND_BREAKDOWN: [usize; 3] = [5, 10, 15];

pub struct SimulationResult {
  pub winner_total_card_counts: HashMap<Card, usize>,
  pub p_present_win: HashMap<Card, f64>,
  pub p_present_loss: HashMap<Card, f64>,
  pub p_present_win_by_round: [(usize, HashMap<Card, f64>); ROUND_BREAKDOWN.len()],
  pub p_present_loss_by_round: [(usize, HashMap<Card, f64>); ROUND_BREAKDOWN.len()],
}

pub struct SimulationAccumulator {
  // Total card counts for the winner HashMap<Card, count>
  pub winner_total_card_counts: Mutex<HashMap<Card, usize>>,
  // Present card counts for the winner HashMap<Card, count>
  pub win_present_card_counts: Mutex<HashMap<Card, usize>>,
  // Present card counts for the other players HashMap<Card, count>
  pub loss_present_card_counts: Mutex<HashMap<Card, usize>>,
  // Present card counts for the winner by round (round number, HashMap<Card, count>)
  pub win_present_card_counts_by_round:
    Mutex<[(usize, HashMap<Card, usize>); ROUND_BREAKDOWN.len()]>,
  // Present card counts for the other players by round (round number, HashMap<Card, count>)
  pub loss_present_card_counts_by_round:
    Mutex<[(usize, HashMap<Card, usize>); ROUND_BREAKDOWN.len()]>,
}

impl SimulationAccumulator {
  pub fn new() -> Self {
    Self {
      winner_total_card_counts: Mutex::new(HashMap::new()),
      win_present_card_counts: Mutex::new(HashMap::new()),
      loss_present_card_counts: Mutex::new(HashMap::new()),
      win_present_card_counts_by_round: Mutex::new(std::array::from_fn(|i| {
        (ROUND_BREAKDOWN[i], HashMap::new())
      })),
      loss_present_card_counts_by_round: Mutex::new(std::array::from_fn(|i| {
        (ROUND_BREAKDOWN[i], HashMap::new())
      })),
    }
  }

  /// Accumulates the result of a single simulation (game run)
  pub fn accumulate(&self, result: &GameResult) {
    self.accumulate_winner_total_card_counts(result);
    self.accumulate_win_present_card_counts(result);
    self.accumulate_loss_present_card_counts(result);
    self.accumulate_win_present_card_counts_by_round(result);
    self.accumulate_loss_present_card_counts_by_round(result);
  }

  fn accumulate_winner_total_card_counts(&self, result: &GameResult) {
    let mut total = self.winner_total_card_counts.lock().unwrap();
    for OwnedCard { card, .. } in result.player_cards[result.winner_index].iter() {
      *total.entry(*card).or_insert(0) += 1;
    }
  }

  fn accumulate_win_present_card_counts(&self, result: &GameResult) {
    let mut presence = self.win_present_card_counts.lock().unwrap();
    let distinct_cards = result.player_cards[result.winner_index]
      .iter()
      .map(|OwnedCard { card, .. }| card)
      .collect::<HashSet<&Card>>();
    for card in distinct_cards {
      *presence.entry(*card).or_insert(0) += 1;
    }
  }

  fn accumulate_loss_present_card_counts(&self, result: &GameResult) {
    let mut loss_presence = self.loss_present_card_counts.lock().unwrap();
    for player_index in 0..result.player_cards.len() {
      if player_index == result.winner_index {
        continue;
      }
      let distinct_cards = result.player_cards[player_index]
        .iter()
        .map(|OwnedCard { card, .. }| card)
        .collect::<HashSet<&Card>>();
      for card in distinct_cards {
        *loss_presence.entry(*card).or_insert(0) += 1;
      }
    }
  }

  fn accumulate_win_present_card_counts_by_round(&self, result: &GameResult) {
    let mut win_present_card_counts_by_round =
      self.win_present_card_counts_by_round.lock().unwrap();
    for (index, round) in ROUND_BREAKDOWN.iter().enumerate() {
      let filtered_distinct_cards = result.player_cards[result.winner_index]
        .iter()
        .filter(|OwnedCard { bought_round, .. }| *bought_round < *round as u8)
        .map(|OwnedCard { card, .. }| card)
        .collect::<HashSet<&Card>>();
      for card in filtered_distinct_cards {
        *win_present_card_counts_by_round[index]
          .1
          .entry(*card)
          .or_insert(0) += 1;
      }
    }
  }

  fn accumulate_loss_present_card_counts_by_round(&self, result: &GameResult) {
    let mut loss_present_card_counts_by_round =
      self.loss_present_card_counts_by_round.lock().unwrap();
    for (index, round) in ROUND_BREAKDOWN.iter().enumerate() {
      for player_index in 0..result.player_cards.len() {
        if player_index == result.winner_index {
          continue;
        }
        let filtered_distinct_cards = result.player_cards[player_index]
          .iter()
          .filter(|OwnedCard { bought_round, .. }| *bought_round < *round as u8)
          .map(|OwnedCard { card, .. }| card)
          .collect::<HashSet<&Card>>();
        for card in filtered_distinct_cards {
          *loss_present_card_counts_by_round[index]
            .1
            .entry(*card)
            .or_insert(0) += 1;
        }
      }
    }
  }

  pub fn finalize(self, sim_count: usize, num_players: usize) -> SimulationResult {
    let winner_total_card_counts = self.winner_total_card_counts.into_inner().unwrap();

    let win_present_card_counts = self.win_present_card_counts.into_inner().unwrap();
    let mut p_present_win: HashMap<Card, f64> = HashMap::new();
    for card in Card::iter() {
      p_present_win.insert(
        card,
        *win_present_card_counts.get(&card).unwrap_or(&0) as f64 / sim_count as f64,
      );
    }

    let loss_present_card_counts = self.loss_present_card_counts.into_inner().unwrap();
    let mut p_present_loss: HashMap<Card, f64> = HashMap::new();
    for card in Card::iter() {
      p_present_loss.insert(
        card,
        *loss_present_card_counts.get(&card).unwrap_or(&0) as f64
          / (sim_count * (num_players - 1)) as f64,
      );
    }

    let win_present_card_counts_by_round =
      self.win_present_card_counts_by_round.into_inner().unwrap();
    let mut p_present_win_by_round: [(usize, HashMap<Card, f64>); ROUND_BREAKDOWN.len()] =
      std::array::from_fn(|i| (ROUND_BREAKDOWN[i], HashMap::new()));
    for (index, round) in ROUND_BREAKDOWN.iter().enumerate() {
      for card in Card::iter() {
        p_present_win_by_round[index].1.insert(
          card,
          *win_present_card_counts_by_round[index]
            .1
            .get(&card)
            .unwrap_or(&0) as f64
            / sim_count as f64,
        );
      }
    }

    let loss_present_card_counts_by_round =
      self.loss_present_card_counts_by_round.into_inner().unwrap();
    let mut p_present_loss_by_round: [(usize, HashMap<Card, f64>); ROUND_BREAKDOWN.len()] =
      std::array::from_fn(|i| (ROUND_BREAKDOWN[i], HashMap::new()));
    for (index, round) in ROUND_BREAKDOWN.iter().enumerate() {
      for card in Card::iter() {
        p_present_loss_by_round[index].1.insert(
          card,
          *loss_present_card_counts_by_round[index]
            .1
            .get(&card)
            .unwrap_or(&0) as f64
            / (sim_count * (num_players - 1)) as f64,
        );
      }
    }

    SimulationResult {
      winner_total_card_counts,
      p_present_win,
      p_present_loss,
      p_present_win_by_round,
      p_present_loss_by_round,
    }
  }
}
