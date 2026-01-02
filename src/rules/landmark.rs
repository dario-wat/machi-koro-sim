use crate::game::Game;
use crate::models::{CardCategory, Landmark};
use strum::IntoEnumIterator;

/// Activate immediate landmarks (one-time effects when built)
pub fn activate_landmark(landmark: Landmark, game: &mut Game, owner_index: usize) {
  match landmark {
    Landmark::ExhibitHall => {
      game.take_coins_from_opponents_with_more_than_10_coins(owner_index);
    }
    Landmark::FrenchRestaurant => {
      game.take_coins_from_each_opponent(owner_index, 2);
    }
    Landmark::LaunchPad => {
      // you win
    }
    Landmark::Museum => {
      game.take_coins_from_each_opponent_for_each_landmark(owner_index);
    }
    Landmark::Park => {
      game.redistribute_coins_evenly();
    }
    Landmark::Publisher => {
      game.take_coins_from_each_opponent_for_each_card(owner_index, 1, |card_def| {
        card_def.category == CardCategory::Bread
      });
    }
    Landmark::RadioTower => {
      // Take another turn after this one
    }
    Landmark::TvStation => {
      game.take_coins_from_each_opponent_for_each_card(owner_index, 1, |card_def| {
        card_def.category == CardCategory::Cup
      });
    }
    _ => {
      // Infinite landmarks should never reach this function
      unreachable!(
        "activate_landmark called on Infinite landmark: {:?}",
        landmark
      )
    }
  }
}

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
    _ => {
      // Only specific landmarks provide card earnings bonuses
      unreachable!(
        "get_card_earnings_bonus called on landmark that doesn't provide bonuses: {:?}",
        landmark
      )
    }
  }
}

/// Handle trigger-based effects on dice roll
pub fn on_dice_roll(landmark: Landmark, game: &mut Game, owner_index: usize, roll: (u8, u8)) {
  let (roll1, roll2) = roll;
  let is_doubles = roll1 == roll2;

  match landmark {
    Landmark::AmusementPark => {
      if is_doubles {
        // game.take_another_turn();
        // TODO: implement this
      }
    }
    Landmark::MovingCompany => {
      // if you roll doubles, give 1 of your establishments to the player on the right
    }
    Landmark::TechStartup => {
      if roll1 + roll2 == 12 {
        game.get_coins_from_bank(owner_index, 8);
      }
    }
    Landmark::Temple => {
      if is_doubles {
        game.take_coins_from_each_opponent(owner_index, 2);
      }
    }
    _ => {
      // Only specific landmarks trigger on dice roll
      unreachable!(
        "on_dice_roll called on landmark that doesn't trigger on dice roll: {:?}",
        landmark
      )
    }
  }
}

/// Handle trigger-based effects at turn end
pub fn on_turn_end(landmark: Landmark, game: &mut Game, owner_index: usize, built_something: bool) {
  match landmark {
    Landmark::Airport => {
      // if you dont build anything on your turn, get 5 coins from the bank
    }
    _ => {
      // Only specific landmarks trigger at turn end
      unreachable!(
        "on_turn_end called on landmark that doesn't trigger at turn end: {:?}",
        landmark
      )
    }
  }
}

/// Handle trigger-based effects after card activation
pub fn on_after_card_activation(
  landmark: Landmark,
  game: &mut Game,
  owner_index: usize,
  coins_received: u8,
) {
  match landmark {
    Landmark::Charterhouse => {
      // if you roll two dice and receive no coins, get 3 coins from the bank
    }
    _ => {
      // Only specific landmarks trigger after card activation
      unreachable!(
        "on_after_card_activation called on landmark that doesn't trigger after card activation: {:?}",
        landmark
      )
    }
  }
}

/// Get landmark build cost reduction
pub fn get_landmark_cost_reduction(landmark: Landmark, target_landmark: Landmark) -> u8 {
  match landmark {
    Landmark::LoanOffice => {
      // you can buy only when the only player withou landmark. reduce build
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
    _ => {
      // Only specific landmarks provide cost reduction
      unreachable!(
        "get_landmark_cost_reduction called on landmark that doesn't reduce costs: {:?}",
        landmark
      )
    }
  }
}

pub fn build_landmark_deck() -> Vec<Landmark> {
  Landmark::iter().collect()
}
