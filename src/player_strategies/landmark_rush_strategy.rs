use std::collections::HashSet;

use crate::game::Game;
use crate::models::Card;
use crate::player_strategies::player_strategy::{
  DiceRollDecision, ExchangeEstablishmentDecision, GiveEstablishmentDecision, PlayerStrategy,
  PurchaseDecision,
};
use crate::player_strategies::RandomStrategy;

pub struct LandmarkRushStrategy {
  random: RandomStrategy,
}

impl LandmarkRushStrategy {
  pub fn new() -> Self {
    Self {
      random: RandomStrategy::new(),
    }
  }
}

impl PlayerStrategy for LandmarkRushStrategy {
  // Override: Buy landmark first if available
  fn decide_purchase(&mut self, game: &Game) -> PurchaseDecision {
    if let Some(landmark) = game.get_affordable_landmarks().first() {
      return PurchaseDecision::BuyLandmark(*landmark);
    }
    self.random.decide_purchase(game)
  }

  // Delegate: Use RandomStrategy for dice rolls
  fn decide_dice_roll(&mut self, game: &Game) -> DiceRollDecision {
    self.random.decide_dice_roll(game)
  }

  // Delegate: Use RandomStrategy for exchanges
  fn decide_exchange_establishment(&mut self, game: &Game) -> ExchangeEstablishmentDecision {
    self.random.decide_exchange_establishment(game)
  }

  // Delegate: Use RandomStrategy for giving establishments
  fn decide_give_establishment(&mut self, game: &Game) -> GiveEstablishmentDecision {
    self.random.decide_give_establishment(game)
  }
}
