use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::{
  game::Game,
  player_strategies::{
    player_strategy::{DiceRollDecision, PurchaseDecision},
    strategy_utils::decide_purchase_randomly,
    PlayerStrategy,
  },
};

pub struct OptimizedStrategy {
  rng: StdRng,
}

impl OptimizedStrategy {
  pub fn new() -> Self {
    Self {
      rng: StdRng::from_entropy(),
    }
  }
}

impl PlayerStrategy for OptimizedStrategy {
  fn decide_dice_roll(&mut self, game: &Game) -> DiceRollDecision {
    if self.rng.gen_bool(0.5) {
      DiceRollDecision::RollOne
    } else {
      DiceRollDecision::RollTwo
    }
  }

  fn decide_purchase(&mut self, game: &Game) -> PurchaseDecision {
    decide_purchase_randomly(&mut self.rng, game)
  }
}
