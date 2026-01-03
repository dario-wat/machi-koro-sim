use crate::game::Game;
use crate::models::{CardCategory, Landmark};
use strum::IntoEnumIterator;

/// Activate immediate landmarks (one-time effects when built)
pub fn activate_landmark(landmark: Landmark, game: &mut Game) {
  match landmark {
    Landmark::ExhibitHall => {
      game.take_coins_from_opponents_with_more_than_10_coins(game.current_player);
    }
    Landmark::FrenchRestaurant => {
      game.take_coins_from_each_opponent(game.current_player, 2);
    }
    Landmark::LaunchPad => {
      // you win. do nothing, winning is handled by the engine
    }
    Landmark::Museum => {
      game.take_coins_from_each_opponent_for_each_landmark(game.current_player);
    }
    Landmark::Park => {
      game.redistribute_coins_evenly();
    }
    Landmark::Publisher => {
      game.take_coins_from_each_opponent_for_each_card(game.current_player, 1, |card_def| {
        card_def.category == CardCategory::Bread
      });
    }
    Landmark::RadioTower => {
      game.take_another_turn_after_this_one();
    }
    Landmark::TvStation => {
      game.take_coins_from_each_opponent_for_each_card(game.current_player, 1, |card_def| {
        card_def.category == CardCategory::Cup
      });
    }
    _ => {}
  }
}

// TODO use this
/// Get bonus coins for card earnings based on passive landmark modifiers
pub fn get_card_earnings_bonus(landmark: Landmark, card_category: CardCategory) -> u8 {
  match landmark {
    Landmark::FarmersMarket => {
      if card_category == CardCategory::Wheat {
        1
      } else {
        0
      }
    }
    Landmark::Forge => {
      // gear establishments earn +1 coin
      if card_category == CardCategory::Gear {
        1
      } else {
        0
      }
    }
    Landmark::SodaBottlingPlant => {
      // your cup cards earn +1 coin when activated
      if card_category == CardCategory::Cup {
        1
      } else {
        0
      }
    }
    Landmark::ShoppingMall => {
      // your bread cards earn +1 coin when activated
      if card_category == CardCategory::Bread {
        1
      } else {
        0
      }
    }
    _ => 0,
  }
}

/// Handle trigger-based effects on dice roll
pub fn on_dice_roll(landmark: Landmark, game: &mut Game, roll: (u8, u8)) {
  let (roll1, roll2) = roll;
  let is_doubles = roll1 == roll2;

  match landmark {
    Landmark::AmusementPark => {
      if is_doubles {
        game.take_another_turn_after_this_one();
      }
    }
    Landmark::MovingCompany => {
      // if you roll doubles, give 1 of your establishments to the player on the right
    }
    Landmark::TechStartup => {
      if roll1 + roll2 == 12 {
        game.get_coins_from_bank(game.current_player, 8);
      }
    }
    Landmark::Temple => {
      if is_doubles {
        game.take_coins_from_each_opponent(game.current_player, 2);
      }
    }
    _ => {}
  }
}

// TODO maybe better handling of extra state
/// Handle trigger-based effects at turn end
pub fn on_turn_end(landmark: Landmark, game: &mut Game, built_something: bool) {
  match landmark {
    Landmark::Airport => {
      if !built_something {
        game.get_coins_from_bank(game.current_player, 5);
      }
    }
    _ => {}
  }
}

// TODO use this
/// Handle trigger-based effects after card activation
pub fn on_after_card_activation(landmark: Landmark, game: &mut Game, coins_received: u8) {
  match landmark {
    Landmark::Charterhouse => {
      // if you roll two dice and receive no coins, get 3 coins from the bank
    }
    _ => {}
  }
}

/// Get landmark build cost reduction
pub fn get_landmark_cost_reduction(landmark: Landmark, target_landmark: Landmark) -> u8 {
  match landmark {
    Landmark::LoanOffice => {
      // you can buy only when the only player without landmark. reduce build
      // cost of landmarks by 2 coins
      2
    }
    Landmark::Observatory => {
      // reduce build cost of launch pad by 5 coins
      if target_landmark == Landmark::LaunchPad {
        5
      } else {
        0
      }
    }
    _ => 0,
  }
}

pub fn build_landmark_deck() -> Vec<Landmark> {
  Landmark::iter().collect()
}
