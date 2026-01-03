use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::{game::Game, player_strategies::player_strategy::PurchaseDecision};

/// Player chooses a random card, landmark or nothing to buy. Each options has equal probability.
/// E.g. if player can afford 2 cards and 1 landmark, each option has a 1/4 probability.
pub fn decide_purchase_randomly(rng: &mut StdRng, game: &Game) -> PurchaseDecision {
  // Pre-allocate: max 5 cards from each deck + 5 landmarks + 1 nothing = ~16 options
  let mut options = Vec::with_capacity(16);

  for card in game
    .less_than_7_face_up
    .keys()
    .filter(|card| game.current_player_can_afford_card(card))
  {
    options.push(PurchaseDecision::BuyCard(*card));
  }

  for card in game
    .greater_than_6_face_up
    .keys()
    .filter(|card| game.current_player_can_afford_card(card))
  {
    options.push(PurchaseDecision::BuyCard(*card));
  }

  for landmark in game
    .landmark_face_up
    .iter()
    .filter(|landmark| game.current_player_can_afford_landmark(landmark))
  {
    options.push(PurchaseDecision::BuyLandmark(*landmark));
  }

  options.push(PurchaseDecision::BuyNothing);

  *options.choose(rng).unwrap_or(&PurchaseDecision::BuyNothing)
}
