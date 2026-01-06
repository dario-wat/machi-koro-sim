use std::collections::HashSet;

use crate::game::Game;
use crate::models::Card;
use crate::player_strategies::player_strategy::{
  DiceRollDecision, ExchangeEstablishmentDecision, GiveEstablishmentDecision, PlayerStrategy,
  PurchaseDecision,
};
use crate::player_strategies::RandomStrategy;

pub struct GreedyBestCardStrategy {
  random: RandomStrategy, // Composition: contains a RandomStrategy
}

impl GreedyBestCardStrategy {
  pub fn new() -> Self {
    Self {
      random: RandomStrategy::new(),
    }
  }
}

const CARDS_TO_BUY: &[Card] = &[Card::ShoppingDistrict, Card::Vineyard, Card::FlowerGarden];

impl PlayerStrategy for GreedyBestCardStrategy {
  // Override: Always buy the best card (greedy logic)
  fn decide_purchase(&mut self, game: &Game) -> PurchaseDecision {
    let affordable_cards: HashSet<Card> = game.get_affordable_cards().into_iter().collect();
    for card in CARDS_TO_BUY {
      if affordable_cards.contains(card) {
        return PurchaseDecision::BuyCard(*card);
      }
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
