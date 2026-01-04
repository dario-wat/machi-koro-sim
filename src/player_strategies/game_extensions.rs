use crate::{
  game::Game,
  models::{Card, Landmark},
};

// There can be at most 10 cards available for purchase. 5 from 1-6 and 5 from 2-12.
const MAX_AFFORDABLE_CARDS: usize = 10;
// There can be at most 5 landmarks available for purchase.
const MAX_AFFORDABLE_LANDMARKS: usize = 5;
// There can be at most 60 cards that the opponents own.
const MAX_OPPONENTS_CARDS: usize = 60;

/// Additional extensions to the Game struct that are used by the player strategies.
impl Game {
  /// List of cards that the player is able to buy
  pub fn get_affordable_cards(&self) -> Vec<Card> {
    let mut cards = Vec::with_capacity(MAX_AFFORDABLE_CARDS);

    for card in self
      .less_than_7_face_up
      .keys()
      .filter(|card| self.current_player_can_afford_card(card))
    {
      cards.push(*card);
    }

    for card in self
      .greater_than_6_face_up
      .keys()
      .filter(|card| self.current_player_can_afford_card(card))
    {
      cards.push(*card);
    }

    cards
  }

  /// List of landmarks that the player is able to buy
  pub fn get_affordable_landmarks(&self) -> Vec<Landmark> {
    let mut landmarks = Vec::with_capacity(MAX_AFFORDABLE_LANDMARKS);
    for landmark in self
      .landmark_face_up
      .iter()
      .filter(|landmark| self.current_player_can_afford_landmark(landmark))
    {
      landmarks.push(*landmark);
    }
    landmarks
  }

  /// List of cards that the opponents own, along with their index in the players array
  pub fn get_opponents_cards(&self) -> Vec<(Card, usize)> {
    let mut cards = Vec::with_capacity(MAX_OPPONENTS_CARDS);
    for (index, player) in self.players.iter().enumerate() {
      if index != self.current_player {
        for card in player.cards.iter() {
          cards.push((card.card, index));
        }
      }
    }
    cards
  }
}
