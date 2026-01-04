use crate::game::Game;
use crate::models::player::OwnedCard;
use crate::player_strategies::player_strategy::{
  DiceRollDecision, ExchangeEstablishmentDecision, GiveEstablishmentDecision, PlayerStrategy,
  PurchaseDecision,
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
  fn decide_dice_roll(&mut self, game: &Game) -> DiceRollDecision {
    if self.rng.gen_bool(0.5) {
      DiceRollDecision::RollOne
    } else {
      DiceRollDecision::RollTwo
    }
  }

  /// Player chooses a random card, landmark or nothing to buy. Each options has equal probability.
  /// E.g. if player can afford 2 cards and 1 landmark, each option has a 1/4 probability.
  fn decide_purchase(&mut self, game: &Game) -> PurchaseDecision {
    // Pre-allocate: max 5 cards from each deck + 5 landmarks + 1 nothing = ~16 options
    let mut options = Vec::with_capacity(16);
    for card in game.get_affordable_cards() {
      options.push(PurchaseDecision::BuyCard(card));
    }
    for landmark in game.get_affordable_landmarks() {
      options.push(PurchaseDecision::BuyLandmark(landmark));
    }
    options.push(PurchaseDecision::BuyNothing);
    *options
      .choose(&mut self.rng)
      .unwrap_or(&PurchaseDecision::BuyNothing)
  }

  /// Player chooses a random card of their own or no card. If a card is chosen, then the player
  /// chooses a random card from any of the opponents. Each option has equal probability.
  fn decide_exchange_establishment(&mut self, game: &Game) -> ExchangeEstablishmentDecision {
    let current_player = &game.players[game.current_player];
    if current_player.cards.is_empty() {
      return ExchangeEstablishmentDecision::NoExchange;
    }

    let num_cards = current_player.cards.len();
    let choice_index = self.rng.gen_range(0..=num_cards);
    if choice_index == num_cards {
      return ExchangeEstablishmentDecision::NoExchange;
    }

    let OwnedCard {
      card: card_to_exchange,
      ..
    } = current_player.cards[choice_index];

    let (opponent_card, opponent_index) =
      *game.get_opponents_cards().choose(&mut self.rng).unwrap();

    ExchangeEstablishmentDecision::Exchange(card_to_exchange, opponent_index, opponent_card)
  }

  /// Player chooses a random card of their own to give.
  fn decide_give_establishment(&mut self, game: &Game) -> GiveEstablishmentDecision {
    if game.players[game.current_player].cards.is_empty() {
      return GiveEstablishmentDecision::NoGive;
    }

    let owned_card = game.players[game.current_player]
      .cards
      .choose(&mut self.rng)
      .unwrap();
    GiveEstablishmentDecision::Give(owned_card.card)
  }
}
