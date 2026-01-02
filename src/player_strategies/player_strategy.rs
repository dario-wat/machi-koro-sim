use crate::game::Game;
use crate::models::{Card, Landmark};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurchaseDecision {
  BuyCard(Card),
  BuyLandmark(Landmark),
  BuyNothing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiceRollDecision {
  RollOne,
  RollTwo,
}

/// Trait (interface) that all player strategies must implement
/// This defines the contract that any player strategy must fulfill
pub trait PlayerStrategy {
  fn decide_dice_roll(&mut self, game: &Game, owner_index: usize) -> DiceRollDecision;
  fn decide_purchase(&mut self, game: &Game, owner_index: usize) -> PurchaseDecision;
}
