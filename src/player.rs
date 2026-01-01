use crate::card::Card;
use crate::landmark::Landmark;

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
