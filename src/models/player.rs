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
}
