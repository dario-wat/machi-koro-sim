use crate::game::Game;
use crate::models::{Card, CardCategory, CardColor, CardDef};

pub fn activate_card(card: Card, game: &mut Game, owner_index: usize) {
  match card {
    Card::SushiBar => {
      take_coins_from_active_player(game, owner_index, 3);
    }

    Card::WheatField => {
      get_coins_from_bank(game, owner_index, 1);
    }

    Card::Vineyard => {
      get_coins_from_bank(game, owner_index, 2);
    }

    Card::Bakery => {
      get_coins_from_bank(game, owner_index, 2);
    }

    Card::Cafe => {
      take_coins_from_active_player(game, owner_index, 2);
    }

    Card::FlowerGarden => {
      get_coins_from_bank(game, owner_index, 2);
    }

    Card::ConvenienceStore => {
      get_coins_from_bank(game, owner_index, 3);
    }

    Card::Forest => {
      get_coins_from_bank(game, owner_index, 2);
    }

    Card::CornField => {
      get_coins_from_bank(game, owner_index, 3);
    }

    Card::HamburgerStand => {
      take_coins_from_active_player(game, owner_index, 2);
    }

    Card::FamilyRestaurant => {
      take_coins_from_active_player(game, owner_index, 2);
    }

    Card::AppleOrchard => {
      get_coins_from_bank(game, owner_index, 3);
    }

    Card::Mine => {
      get_coins_from_bank(game, owner_index, 6);
    }

    Card::FlowerShop => {
      get_coins_from_bank_for_each_card(game, owner_index, 3, |card_def| {
        card_def.category == CardCategory::Flower
      });
    }

    Card::BusinessCenter => {
      // TODO: exchange establishment
    }

    Card::Stadium => {
      for player_index in 0..game.players.len() {
        if player_index != owner_index {
          move_coins_between_players(game, game.current_player as usize, player_index, 3);
        }
      }
    }

    Card::FurnitureFactory => {
      get_coins_from_bank_for_each_card(game, owner_index, 4, |card_def| {
        card_def.color == CardColor::Green
      });
    }

    Card::ShoppingDistrict => {
      for player_index in 0..game.players.len() {
        if player_index != owner_index && game.players[player_index].coins > 10 {
          let coins_to_take = game.players[player_index].coins / 2;
          move_coins_between_players(game, player_index, owner_index, coins_to_take);
        }
      }
    }

    Card::Winery => {
      get_coins_from_bank_for_each_card(game, owner_index, 3, |card_def| {
        card_def.category == CardCategory::Fruit
      });
    }

    Card::FoodWarehouse => {
      get_coins_from_bank_for_each_card(game, owner_index, 2, |card_def| {
        card_def.category == CardCategory::Cup
      });
    }
  }
}

#[inline]
fn get_coins_from_bank(game: &mut Game, owner_index: usize, amount: u8) {
  game.players[owner_index].coins += amount;
}

#[inline]
fn get_coins_from_bank_for_each_card(
  game: &mut Game,
  owner_index: usize,
  amount: u8,
  predicate: fn(&CardDef) -> bool,
) {
  let coins_to_get: u8 = game.players[owner_index]
    .cards
    .iter()
    .filter(|card| predicate(&card.def()))
    .map(|_card| amount)
    .sum();
  game.players[owner_index].coins += coins_to_get;
}

#[inline]
fn move_coins_between_players(game: &mut Game, from_index: usize, to_index: usize, amount: u8) {
  let from_coins = game.players[from_index].coins;
  let coins_to_move = std::cmp::min(from_coins, amount);
  game.players[from_index].coins -= coins_to_move;
  game.players[to_index].coins += coins_to_move;
}

#[inline]
fn take_coins_from_active_player(game: &mut Game, owner_index: usize, amount: u8) {
  move_coins_between_players(game, game.current_player as usize, owner_index, amount);
}

const DECK_COMPOSITION: &[(Card, u8)] = &[
  (Card::SushiBar, 5),
  (Card::WheatField, 5),
  (Card::Vineyard, 5),
  (Card::Bakery, 5),
  (Card::Cafe, 5),
  (Card::FlowerGarden, 5),
  (Card::ConvenienceStore, 5),
  (Card::Forest, 5),
  (Card::CornField, 5),
  (Card::HamburgerStand, 5),
  (Card::FamilyRestaurant, 5),
  (Card::AppleOrchard, 5),
  (Card::Mine, 5),
  (Card::FlowerShop, 3),
  (Card::BusinessCenter, 3),
  (Card::Stadium, 3),
  (Card::FurnitureFactory, 3),
  (Card::ShoppingDistrict, 3),
  (Card::Winery, 3),
  (Card::FoodWarehouse, 3),
];

pub fn build_less_than_7_deck() -> Vec<Card> {
  DECK_COMPOSITION
    .iter()
    .filter(|(card, _copies)| {
      let def = card.def();
      def.activation.iter().all(|&activation| activation <= 6)
    })
    .flat_map(|(card, copies)| std::iter::repeat(card.clone()).take(*copies as usize))
    .collect()
}

pub fn build_greater_than_6_deck() -> Vec<Card> {
  DECK_COMPOSITION
    .iter()
    .filter(|(card, _copies)| {
      let def = card.def();
      def.activation.iter().all(|&activation| activation > 6)
    })
    .flat_map(|(card, copies)| std::iter::repeat(card.clone()).take(*copies as usize))
    .collect()
}
