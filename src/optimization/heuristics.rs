use crate::{
  engine::Engine,
  math::p_card_activation,
  models::{card::CardEffect, player::OwnedCard, Card},
};

impl Engine {
  pub fn expected_coin_gain_per_round_for_card(&self, card: Card) -> u16 {
    let effect = card.def().effect;
    match effect {
      CardEffect::TakeCoinsFromActivePlayer(amount) => {
        let expected_gain =
          (self.game.players.len() - 1) as f64 * p_card_activation(card) * amount as f64;
        expected_gain.round() as u16
      }
      CardEffect::TakeCoinsFromEachOpponent(amount) => {
        let expected_gain =
          (self.game.players.len() - 1) as f64 * p_card_activation(card) * amount as f64;
        expected_gain.round() as u16
      }
      CardEffect::TakeCoinsFromEachOpponentWithMoreThan10Coins => {
        let opponents_with_more_than_10_coins_sum = self
          .game
          .players
          .iter()
          .enumerate()
          .filter(|(index, player)| *index != self.game.current_player && player.coins > 10)
          .map(|(_, player)| player.coins / 2)
          .sum::<u16>();
        let expected_gain = opponents_with_more_than_10_coins_sum as f64 * p_card_activation(card);
        expected_gain.round() as u16
      }
      CardEffect::GetCoinsFromBank(amount) => {
        let expected_gain =
          self.game.players.len() as f64 * p_card_activation(card) * amount as f64;
        expected_gain.round() as u16
      }
      CardEffect::GetCoinsFromBankForEachCardCategory(amount, category) => {
        let card_count = self
          .game
          .get_current_player()
          .cards
          .iter()
          .filter(|OwnedCard { card, .. }| card.def().category == category)
          .count();
        let expected_gain = p_card_activation(card) * amount as f64 * card_count as f64;
        expected_gain.round() as u16
      }
      CardEffect::GetCoinsFromBankForEachCardColor(amount, color) => {
        let card_count = self
          .game
          .get_current_player()
          .cards
          .iter()
          .filter(|OwnedCard { card, .. }| card.def().color == color)
          .count();
        let expected_gain = p_card_activation(card) * amount as f64 * card_count as f64;
        expected_gain.round() as u16
      }
      CardEffect::ExchangeEstablishment => 0,
    }
  }
}
