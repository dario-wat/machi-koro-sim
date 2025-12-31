use std::collections::HashMap;

use card::build_greater_than_6_deck;
use card::build_less_than_7_deck;
use card::Card;
use landmark::build_landmark_deck;
use landmark::Landmark;
use rng::Rng;

pub struct Game {
  rng: Rng,

  current_player: u8,
  current_round: u8,

  less_than_7_deck: Vec<Card>,
  greater_than_6_deck: Vec<Card>,
  landmark_deck: Vec<Landmark>,

  // Face-up cards: HashMap<Card, count>
  less_than_7_face_up: HashMap<Card, u8>,
  greater_than_6_face_up: HashMap<Card, u8>,
  landmark_face_up: HashMap<Landmark, u8>,
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

    let mut game = Self {
      current_player: 0,
      current_round: 0,
      rng,
      less_than_7_deck,
      greater_than_6_deck,
      landmark_deck,
      less_than_7_face_up: HashMap::new(),
      greater_than_6_face_up: HashMap::new(),
      landmark_face_up: HashMap::new(),
    };

    // Initialize face-up cards (5 for each deck)
    game.refill_face_up_cards();
    game
  }

  /// Maintain 5 face-up cards for a card deck
  fn refill_face_up_cards_card(&mut self, deck: &mut Vec<Card>, face_up: &mut HashMap<Card, u8>) {
    while face_up.len() < 5 && !deck.is_empty() {
      let card = deck.remove(0);
      *face_up.entry(card).or_insert(0) += 1;
    }
  }

  /// Maintain 5 face-up cards for landmark deck
  fn refill_face_up_cards_landmark(
    &mut self,
    deck: &mut Vec<Landmark>,
    face_up: &mut HashMap<Landmark, u8>,
  ) {
    while face_up.len() < 5 && !deck.is_empty() {
      let landmark = deck.remove(0);
      *face_up.entry(landmark).or_insert(0) += 1;
    }
  }

  /// Refill all face-up card areas to maintain 5 unique cards each
  pub fn refill_face_up_cards(&mut self) {
    self.refill_face_up_cards_card(&mut self.less_than_7_deck, &mut self.less_than_7_face_up);
    self.refill_face_up_cards_card(
      &mut self.greater_than_6_deck,
      &mut self.greater_than_6_face_up,
    );
    self.refill_face_up_cards_landmark(&mut self.landmark_deck, &mut self.landmark_face_up);
  }
}
