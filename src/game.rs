use std::collections::HashMap;

use crate::models::landmark::LandmarkType;
use crate::models::{Card, Landmark, Player};
use crate::rng::Rng;
use crate::rules::card::build_greater_than_6_deck;
use crate::rules::card::build_less_than_7_deck;
use crate::rules::landmark::build_landmark_deck;

const LANDMARKS_TO_WIN: usize = 3;

#[derive(Clone)]
pub struct Game {
  pub rng: Rng,
  pub seed: u64,

  pub current_player: usize,
  pub current_turn: usize,

  pub players: Vec<Player>,

  // Remaining cards in the deck
  pub less_than_7_deck: Vec<Card>,
  pub greater_than_6_deck: Vec<Card>,
  pub landmark_deck: Vec<Landmark>,

  // Face-up cards: HashMap<Card, count>
  pub less_than_7_face_up: HashMap<Card, u8>,
  pub greater_than_6_face_up: HashMap<Card, u8>,
  pub landmark_face_up: Vec<Landmark>,

  // Cached active landmarks (infinite landmarks that apply to all players)
  active_landmarks_cache: Vec<Landmark>,

  // Private state describing some game aspects
  take_another_turn: bool,
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
      current_turn: 0,
      players: Vec::new(),
      less_than_7_deck,
      greater_than_6_deck,
      landmark_deck,
      less_than_7_face_up: HashMap::new(),
      greater_than_6_face_up: HashMap::new(),
      landmark_face_up: Vec::new(),
      active_landmarks_cache: Vec::new(),
      take_another_turn: false,
    };

    // Initialize face-up cards (5 for each deck)
    game.refill_face_up_cards();
    game
  }

  pub fn get_round(&self) -> usize {
    self.current_turn / self.players.len()
  }

  pub fn winner(&self) -> Option<usize> {
    for (index, player) in self.players.iter().enumerate() {
      if player.landmarks.len() == LANDMARKS_TO_WIN
        || player
          .landmarks
          .iter()
          .any(|landmark| *landmark == Landmark::LaunchPad)
      {
        return Some(index);
      }
    }
    None
  }

  pub fn current_player_can_afford_card(&self, card: &Card) -> bool {
    self.players[self.current_player].can_afford_card(card)
  }

  pub fn current_player_can_afford_landmark(&self, landmark: &Landmark) -> bool {
    self.players[self.current_player].can_afford_landmark(landmark)
  }

  /// Get all active landmarks (infinite landmarks apply to all players once built)
  /// Returns a reference to the cached list
  pub fn get_active_landmarks(&self) -> &[Landmark] {
    &self.active_landmarks_cache
  }

  pub fn advance_turn(&mut self) {
    if self.take_another_turn {
      self.take_another_turn = false;
      return;
    }
    self.current_player = (self.current_player + 1) % self.players.len();
    self.current_turn += 1;
  }

  /// Reverts the current turn so that the current player can take another turn
  pub fn take_another_turn_after_this_one(&mut self) {
    self.take_another_turn = true;
  }

  pub fn roll_one_die(&mut self) -> u8 {
    self.rng.roll_die()
  }

  pub fn roll_two_dice(&mut self) -> (u8, u8) {
    self.rng.roll_two_dice()
  }

  /// 1. Add card to player's cards
  /// 2. Subtract card cost from player's coins
  /// 3. Remove card from face-up cards
  /// 4. Refill face-up cards
  pub fn buy_card(&mut self, card: Card) {
    self.players[self.current_player].buy_card(card);

    if let Some(count) = self.less_than_7_face_up.get_mut(&card) {
      *count -= 1;
      if *count == 0 {
        self.less_than_7_face_up.remove(&card);
      }
    }
    if let Some(count) = self.greater_than_6_face_up.get_mut(&card) {
      *count -= 1;
      if *count == 0 {
        self.greater_than_6_face_up.remove(&card);
      }
    }
    self.refill_face_up_cards();
  }

  /// 1. Add landmark to player's landmarks
  /// 2. Subtract landmark cost from player's coins
  /// 3. Remove landmark from face-up landmarks
  /// 4. Refill face-up landmarks
  /// 5. Add to active landmarks cache if infinite
  pub fn buy_landmark(&mut self, landmark: Landmark) {
    self.players[self.current_player].buy_landmark(landmark);
    if let Some(pos) = self.landmark_face_up.iter().position(|l| *l == landmark) {
      self.landmark_face_up.remove(pos);
    }
    self.refill_face_up_cards();

    // Add to active landmarks cache if infinite
    if landmark.def().landmark_type == LandmarkType::Infinite {
      self.active_landmarks_cache.push(landmark);
    }
  }

  /// Refill all face-up card areas to maintain 5 unique cards each
  fn refill_face_up_cards(&mut self) {
    while self.less_than_7_face_up.len() < 5 && !self.less_than_7_deck.is_empty() {
      let card = self.less_than_7_deck.pop().unwrap();
      *self.less_than_7_face_up.entry(card).or_insert(0) += 1;
    }
    while self.greater_than_6_face_up.len() < 5 && !self.greater_than_6_deck.is_empty() {
      let card = self.greater_than_6_deck.pop().unwrap();
      *self.greater_than_6_face_up.entry(card).or_insert(0) += 1;
    }
    while self.landmark_face_up.len() < 5 && !self.landmark_deck.is_empty() {
      let landmark = self.landmark_deck.pop().unwrap();
      self.landmark_face_up.push(landmark);
    }
  }

  /// Iterate over other players in reverse order
  pub fn other_players_reverse(&self) -> impl Iterator<Item = usize> {
    let player_count = self.players.len();
    let current = self.current_player;
    (1..player_count).map(move |offset| (current + player_count - offset) % player_count)
  }
}
