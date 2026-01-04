use crate::game::Game;
use crate::models::{Card, CardCategory, CardColor, CardDef};
use crate::player_strategies::player_strategy::ExchangeEstablishmentDecision;
use crate::player_strategies::PlayerStrategy;

pub fn activate_card(
  card: Card,
  game: &mut Game,
  owner_index: usize,
  player_strategy: &mut dyn PlayerStrategy,
) {
  match card {
    Card::SushiBar => {
      game.take_coins_from_active_player(owner_index, 3);
    }

    Card::WheatField => {
      game.get_coins_from_bank(owner_index, 1);
    }

    Card::Vineyard => {
      game.get_coins_from_bank(owner_index, 2);
    }

    Card::Bakery => {
      game.get_coins_from_bank(owner_index, 2);
    }

    Card::Cafe => {
      game.take_coins_from_active_player(owner_index, 2);
    }

    Card::FlowerGarden => {
      game.get_coins_from_bank(owner_index, 2);
    }

    Card::ConvenienceStore => {
      game.get_coins_from_bank(owner_index, 3);
    }

    Card::Forest => {
      game.get_coins_from_bank(owner_index, 2);
    }

    Card::CornField => {
      game.get_coins_from_bank(owner_index, 3);
    }

    Card::HamburgerStand => {
      game.take_coins_from_active_player(owner_index, 2);
    }

    Card::FamilyRestaurant => {
      game.take_coins_from_active_player(owner_index, 2);
    }

    Card::AppleOrchard => {
      game.get_coins_from_bank(owner_index, 3);
    }

    Card::Mine => {
      game.get_coins_from_bank(owner_index, 6);
    }

    Card::FlowerShop => {
      game.get_coins_from_bank_for_each_card(owner_index, 3, |card_def| {
        card_def.category == CardCategory::Flower
      });
    }

    Card::BusinessCenter => {
      let decision = player_strategy.decide_exchange_establishment(game);
      match decision {
        ExchangeEstablishmentDecision::Exchange(card, opponent_index, opponent_card) => {
          game.exchange_establishment(card, opponent_index, opponent_card);
        }
        ExchangeEstablishmentDecision::NoExchange => {}
      }
    }

    Card::Stadium => {
      game.take_coins_from_each_opponent(owner_index, 3);
    }

    Card::FurnitureFactory => {
      game.get_coins_from_bank_for_each_card(owner_index, 4, |card_def| {
        card_def.color == CardColor::Green
      });
    }

    Card::ShoppingDistrict => {
      game.take_coins_from_opponents_with_more_than_10_coins(owner_index);
    }

    Card::Winery => {
      game.get_coins_from_bank_for_each_card(owner_index, 3, |card_def| {
        card_def.category == CardCategory::Fruit
      });
    }

    Card::FoodWarehouse => {
      game.get_coins_from_bank_for_each_card(owner_index, 2, |card_def| {
        card_def.category == CardCategory::Cup
      });
    }
  }
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
