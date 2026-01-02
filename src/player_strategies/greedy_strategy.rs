use crate::game::Game;
use crate::player_strategies::player_strategy::{
  DiceRollDecision, PlayerStrategy, PurchaseDecision,
};

pub struct GreedyStrategy;

impl PlayerStrategy for GreedyStrategy {
  fn decide_dice_roll(&mut self, game: &Game, owner_index: usize) -> DiceRollDecision {
    // TODO: Implement greedy strategy
    DiceRollDecision::RollTwo
  }

  fn decide_purchase(&mut self, game: &Game, owner_index: usize) -> PurchaseDecision {
    // TODO: Implement greedy strategy
    PurchaseDecision::BuyNothing
  }
}
