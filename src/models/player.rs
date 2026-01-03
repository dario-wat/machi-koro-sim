use super::card::Card;
use super::landmark::Landmark;

pub struct Player {
  pub coins: u8,
  pub cards: Vec<Card>,
  pub landmarks: Vec<Landmark>,
}

impl Player {
  pub fn new() -> Self {
    Self {
      coins: 5, // start with 5 coins
      cards: Vec::new(),
      landmarks: Vec::new(),
    }
  }

  pub fn can_afford_card(&self, card: &Card) -> bool {
    self.coins >= card.def().cost
  }

  pub fn can_afford_landmark(&self, landmark: &Landmark) -> bool {
    self.coins >= landmark.def().cost[self.landmarks.len()]
  }

  pub fn buy_card(&mut self, card: Card) {
    if !self.can_afford_card(&card) {
      panic!("Player cannot afford card");
    }
    self.cards.push(card);
    self.coins -= card.def().cost;
  }

  pub fn buy_landmark(&mut self, landmark: Landmark) {
    if !self.can_afford_landmark(&landmark) {
      panic!("Player cannot afford landmark");
    }
    self.coins -= landmark.def().cost[self.landmarks.len()];
    self.landmarks.push(landmark);
  }
}
