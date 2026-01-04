use crate::game::Game;
use crate::models::{Card, CardDef};

/// Shared helper methods for modifying game state
/// Used by both card and landmark rules
impl Game {
  #[inline]
  pub fn get_coins_from_bank(&mut self, owner_index: usize, amount: u16) {
    self.players[owner_index].coins += amount;
  }

  #[inline]
  pub fn get_coins_from_bank_for_each_card(
    &mut self,
    owner_index: usize,
    amount: u16,
    predicate: fn(&CardDef) -> bool,
  ) {
    let coins_to_get: u16 = self.players[owner_index]
      .cards
      .iter()
      .filter(|card| predicate(&card.def()))
      .map(|_card| amount)
      .sum();
    self.players[owner_index].coins += coins_to_get;
  }

  #[inline]
  pub fn redistribute_coins_evenly(&mut self) {
    let total_coins: u16 = self.players.iter().map(|player| player.coins as u16).sum();
    let num_players = self.players.len() as u16;
    let coins_per_player = ((total_coins + num_players - 1) / num_players) as u16;
    for player in self.players.iter_mut() {
      player.coins = coins_per_player;
    }
  }

  #[inline]
  fn move_coins_between_players(&mut self, from_index: usize, to_index: usize, amount: u16) {
    let from_coins = self.players[from_index].coins;
    let coins_to_move = std::cmp::min(from_coins, amount);
    self.players[from_index].coins -= coins_to_move;
    self.players[to_index].coins += coins_to_move;
  }

  #[inline]
  pub fn take_coins_from_active_player(&mut self, owner_index: usize, amount: u16) {
    self.move_coins_between_players(self.current_player as usize, owner_index, amount);
  }

  #[inline]
  pub fn take_coins_from_each_opponent(&mut self, owner_index: usize, amount: u16) {
    for player_index in 0..self.players.len() {
      if player_index != owner_index {
        self.move_coins_between_players(player_index, owner_index, amount);
      }
    }
  }

  #[inline]
  pub fn take_coins_from_opponents_with_more_than_10_coins(&mut self, owner_index: usize) {
    for player_index in 0..self.players.len() {
      if player_index != owner_index && self.players[player_index].coins > 10 {
        let coins_to_take = self.players[player_index].coins / 2;
        self.move_coins_between_players(player_index, owner_index, coins_to_take);
      }
    }
  }

  #[inline]
  pub fn take_coins_from_each_opponent_for_each_landmark(&mut self, owner_index: usize) {
    for player_index in 0..self.players.len() {
      if player_index != owner_index {
        let landmarks_owned = self.players[player_index].landmarks.len() as u16;
        self.move_coins_between_players(player_index, owner_index, 3 * landmarks_owned);
      }
    }
  }

  #[inline]
  pub fn take_coins_from_each_opponent_for_each_card(
    &mut self,
    owner_index: usize,
    amount: u16,
    predicate: fn(&CardDef) -> bool,
  ) {
    for player_index in 0..self.players.len() {
      if player_index != owner_index {
        let coins_to_take = self.players[player_index]
          .cards
          .iter()
          .filter(|card| predicate(&card.def()))
          .map(|_card| amount)
          .sum();
        self.move_coins_between_players(player_index, owner_index, coins_to_take);
      }
    }
  }

  #[inline]
  fn move_cards_between_players(&mut self, from_index: usize, to_index: usize, card: Card) {
    let card_index = self.players[from_index]
      .cards
      .iter()
      .position(|c| *c == card)
      .unwrap();
    self.players[from_index].cards.remove(card_index);
    self.players[to_index].cards.push(card);
  }

  #[inline]
  pub fn exchange_establishment(&mut self, card: Card, opponent_index: usize, opponent_card: Card) {
    self.move_cards_between_players(self.current_player as usize, opponent_index, card);
    self.move_cards_between_players(opponent_index, self.current_player as usize, opponent_card);
  }
}
