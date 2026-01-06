use super::card::Card;
use super::landmark::Landmark;

#[derive(Clone)]
pub struct OwnedCard {
  pub card: Card,
  pub bought_round: u8,
}

#[derive(Clone)]
pub struct OwnedLandmark {
  pub landmark: Landmark,
  pub bought_round: u8,
}

#[derive(Clone)]
pub struct Player {
  pub coins: u16,
  pub cards: Vec<OwnedCard>,
  pub landmarks: Vec<OwnedLandmark>,
  pub dice_rolls: Vec<(u8, u8)>, // (dice roll - 1 or 2, round)
}

impl Player {
  pub fn new() -> Self {
    Self {
      coins: 5, // start with 5 coins
      cards: Vec::new(),
      landmarks: Vec::new(),
      dice_rolls: Vec::new(),
    }
  }

  pub fn can_afford_card(&self, card: &Card) -> bool {
    self.coins >= card.def().cost
  }

  pub fn can_afford_landmark(&self, landmark: &Landmark) -> bool {
    if *landmark == Landmark::LoanOffice && self.landmarks.len() != 0 {
      return false;
    }
    self.coins >= landmark.def().cost[self.landmarks.len()]
  }

  pub fn buy_card(&mut self, card: Card, bought_round: u8) {
    if !self.can_afford_card(&card) {
      panic!("Player cannot afford card");
    }
    self.cards.push(OwnedCard { card, bought_round });
    self.coins -= card.def().cost;
  }

  pub fn buy_landmark(&mut self, landmark: Landmark, bought_round: u8) {
    if !self.can_afford_landmark(&landmark) {
      panic!("Player cannot afford landmark");
    }
    self.coins -= landmark.def().cost[self.landmarks.len()];
    self.landmarks.push(OwnedLandmark {
      landmark,
      bought_round,
    });
  }
}
