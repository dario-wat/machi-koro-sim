use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::player_strategies::player_strategy::{
  ExchangeEstablishmentDecision, GiveEstablishmentDecision,
};
use crate::{
  game::Game,
  player_strategies::{
    player_strategy::{DiceRollDecision, PurchaseDecision},
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

  // TODO placeholder
  fn decide_purchase(&mut self, game: &Game) -> PurchaseDecision {
    PurchaseDecision::BuyNothing
  }

  // TODO placeholder
  fn decide_exchange_establishment(&mut self, game: &Game) -> ExchangeEstablishmentDecision {
    ExchangeEstablishmentDecision::NoExchange
  }

  // TODO placeholder
  fn decide_give_establishment(&mut self, game: &Game) -> GiveEstablishmentDecision {
    GiveEstablishmentDecision::NoGive
  }
}
