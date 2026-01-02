use crate::game::Game;
use crate::player_strategies::player_strategy::{
  DiceRollDecision, PlayerStrategy, PurchaseDecision,
};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

pub struct RandomStrategy {
  rng: StdRng,
}

impl RandomStrategy {
  pub fn new() -> Self {
    Self {
      rng: StdRng::from_entropy(),
    }
  }
}

impl PlayerStrategy for RandomStrategy {
  fn decide_dice_roll(&mut self, game: &Game, owner_index: usize) -> DiceRollDecision {
    if self.rng.gen_bool(0.5) {
      DiceRollDecision::RollOne
    } else {
      DiceRollDecision::RollTwo
    }
  }

  fn decide_purchase(&mut self, game: &Game, owner_index: usize) -> PurchaseDecision {
    let mut options = Vec::new();

    for card in game.less_than_7_face_up.keys() {
      options.push(PurchaseDecision::BuyCard(*card));
    }
    for card in game.greater_than_6_face_up.keys() {
      options.push(PurchaseDecision::BuyCard(*card));
    }
    for landmark in &game.landmark_face_up {
      options.push(PurchaseDecision::BuyLandmark(*landmark));
    }
    options.push(PurchaseDecision::BuyNothing);

    *options
      .choose(&mut self.rng)
      .unwrap_or(&PurchaseDecision::BuyNothing)
  }
}
