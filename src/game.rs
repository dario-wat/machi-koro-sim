use std::collections::HashMap;

use crate::models::card::build_greater_than_6_deck;
use crate::models::card::build_less_than_7_deck;
use crate::models::landmark::build_landmark_deck;
use crate::models::{Card, Landmark, Player};
use crate::rng::Rng;

pub struct Game {
  pub rng: Rng,
  pub seed: u64,

  pub current_player: u8,
  pub current_round: u8,

  pub players: Vec<Player>,

  // Remaining cards in the deck
  pub less_than_7_deck: Vec<Card>,
  pub greater_than_6_deck: Vec<Card>,
  pub landmark_deck: Vec<Landmark>,

  // Face-up cards: HashMap<Card, count>
  pub less_than_7_face_up: HashMap<Card, u8>,
  pub greater_than_6_face_up: HashMap<Card, u8>,
  pub landmark_face_up: Vec<Landmark>,
}

impl Game {
  pub fn new(rng_seed: Option<u64>) -> Self {
    let mut rng = match rng_seed {
      Some(seed) => Rng::new_with_seed(seed),
      None => Rng::new(),
    };

    let mut less_than_7_deck = build_less_than_7_deck();
    let mut greater_than_6_deck = build_greater_than_6_deck();
    let mut landmark_deck = build_landmark_deck();

    rng.shuffle(&mut less_than_7_deck);
    rng.shuffle(&mut greater_than_6_deck);
    rng.shuffle(&mut landmark_deck);

    let seed = rng.get_seed();
    let mut game = Self {
      rng,
      seed,
      current_player: 0,
      current_round: 0,
      players: Vec::new(),
      less_than_7_deck,
      greater_than_6_deck,
      landmark_deck,
      less_than_7_face_up: HashMap::new(),
      greater_than_6_face_up: HashMap::new(),
      landmark_face_up: Vec::new(),
    };

    // Initialize face-up cards (5 for each deck)
    game.refill_face_up_cards();
    game
  }

  /// Maintain 5 face-up cards for less than 7 deck
  fn refill_less_than_7_face_up(&mut self) {
    while self.less_than_7_face_up.len() < 5 && !self.less_than_7_deck.is_empty() {
      let card = self.less_than_7_deck.remove(0);
      *self.less_than_7_face_up.entry(card).or_insert(0) += 1;
    }
  }

  /// Maintain 5 face-up cards for greater than 6 deck
  fn refill_greater_than_6_face_up(&mut self) {
    while self.greater_than_6_face_up.len() < 5 && !self.greater_than_6_deck.is_empty() {
      let card = self.greater_than_6_deck.remove(0);
      *self.greater_than_6_face_up.entry(card).or_insert(0) += 1;
    }
  }

  /// Maintain 5 face-up cards for landmark deck
  fn refill_landmark_face_up(&mut self) {
    while self.landmark_face_up.len() < 5 && !self.landmark_deck.is_empty() {
      let landmark = self.landmark_deck.remove(0);
      self.landmark_face_up.push(landmark);
    }
  }

  /// Refill all face-up card areas to maintain 5 unique cards each
  pub fn refill_face_up_cards(&mut self) {
    self.refill_less_than_7_face_up();
    self.refill_greater_than_6_face_up();
    self.refill_landmark_face_up();
  }

  /// Exchange establishment: owner gives one card to opponent and takes one from opponent
  pub fn decide_exchange_establishment(&mut self, owner_index: usize) {
    let opponent_indices: Vec<usize> = (0..self.players.len())
      .filter(|&i| i != owner_index)
      .collect();
    if opponent_indices.is_empty() {
      return;
    }

    let opponent_index = opponent_indices[self.rng.pick_random_index(opponent_indices.len())];
    if self.players[owner_index].cards.is_empty() || self.players[opponent_index].cards.is_empty() {
      return;
    }

    let opponent_card_index = self
      .rng
      .pick_random_index(self.players[opponent_index].cards.len());
    let opponent_card = self.players[opponent_index]
      .cards
      .remove(opponent_card_index);

    let owner_card_index = self
      .rng
      .pick_random_index(self.players[owner_index].cards.len());
    let owner_card = self.players[owner_index].cards.remove(owner_card_index);

    self.players[opponent_index].cards.push(owner_card);
    self.players[owner_index].cards.push(opponent_card);
  }
}
