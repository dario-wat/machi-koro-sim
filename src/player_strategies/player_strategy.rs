use crate::game::Game;
use crate::models::{Card, Landmark};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurchaseDecision {
  BuyCard(Card),
  BuyLandmark(Landmark),
  BuyNothing,
}

impl fmt::Display for PurchaseDecision {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PurchaseDecision::BuyCard(card) => write!(f, "Buy Card: {}", card.def().name),
      PurchaseDecision::BuyLandmark(landmark) => write!(f, "Buy Landmark: {}", landmark.def().name),
      PurchaseDecision::BuyNothing => write!(f, "Buy Nothing"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiceRollDecision {
  RollOne,
  RollTwo,
}

impl fmt::Display for DiceRollDecision {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DiceRollDecision::RollOne => write!(f, "Roll One Die"),
      DiceRollDecision::RollTwo => write!(f, "Roll Two Dice"),
    }
  }
}

/// Trait (interface) that all player strategies must implement
/// This defines the contract that any player strategy must fulfill
pub trait PlayerStrategy {
  fn decide_dice_roll(&mut self, game: &Game) -> DiceRollDecision;
  fn decide_purchase(&mut self, game: &Game) -> PurchaseDecision;
}
