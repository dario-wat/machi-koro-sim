use std::collections::HashMap;

use crate::{
  debug::{
    debug_print_dice_roll, debug_print_game, debug_print_purchase_decision, debug_print_winner,
  },
  game::Game,
  models::{Card, CardColor, Landmark, Player},
  player_strategies::{
    player_strategy::{DiceRollDecision, PurchaseDecision},
    PlayerStrategy,
  },
  rules::{card::activate_card, landmark as LandmarkRules},
};

const BUY_ONLY_TURNS: usize = 3;
const MIN_PLAYERS: usize = 2;
const MAX_PLAYERS: usize = 4;

pub struct Engine {
  pub game: Game,
  pub player_strategies: Vec<Box<dyn PlayerStrategy>>,
}

impl Engine {
  pub fn new() -> Self {
    Self {
      game: Game::new(None),
      player_strategies: Vec::new(),
    }
  }

  /// Add a player strategy and a player to the game. The two vectors are the same length.
  pub fn add_player_strategy(&mut self, strategy: Box<dyn PlayerStrategy>) {
    self.player_strategies.push(strategy);
    self.game.players.push(Player::new());
  }

  pub fn run(&mut self) {
    if self.player_strategies.len() < MIN_PLAYERS || self.player_strategies.len() > MAX_PLAYERS {
      panic!("Invalid number of players");
    }

    debug_print_game(&self.game);
    loop {
      self.play_turn();
      debug_print_game(&self.game);

      if self.game.winner().is_some() {
        break;
      }
    }

    debug_print_winner(self.game.winner().unwrap());
  }

  pub fn play_turn(&mut self) {
    if (self.game.get_round() < BUY_ONLY_TURNS) {
      self.play_buy_only_turn();
    } else {
      self.play_normal_turn();
    }
    self.game.advance_turn();
  }

  /// First 3 turns are buy only turns
  fn play_buy_only_turn(&mut self) {
    // Phase 1: Buy card
    // Buy phase for the first 3 turns is a little bit different. It does not trigger any effects.
    let decision = self.player_strategies[self.game.current_player].decide_purchase(&self.game);
    debug_print_purchase_decision(decision);

    match decision {
      PurchaseDecision::BuyCard(card) => self.game.buy_card(card),
      PurchaseDecision::BuyLandmark(landmark) => self.game.buy_landmark(landmark),
      PurchaseDecision::BuyNothing => {}
    }
  }

  /// Normal turn has 3 phases: Roll dice, earn income, buy card or landmark
  fn play_normal_turn(&mut self) {
    // Phase 1: Roll dice
    let dice_roll_sum = self.roll_dice_phase();

    // Phase 2: Earn income
    self.earn_income_phase(dice_roll_sum);

    // Phase 3: Buy card or landmark
    self.buy_phase();
  }

  /// Phase 1: Roll dice
  /// In this phase, the player chooses to roll either one or two dice.
  /// The landmarks then trigger their effects based on the dice roll.
  fn roll_dice_phase(&mut self) -> u8 {
    let current_player = self.game.current_player;
    let decision = self.player_strategies[current_player].decide_dice_roll(&self.game);
    let dice_roll = match decision {
      DiceRollDecision::RollOne => (self.game.roll_one_die(), 0),
      DiceRollDecision::RollTwo => self.game.roll_two_dice(),
    };

    debug_print_dice_roll(dice_roll);

    // Clone landmarks to avoid borrow checker issues
    let active_landmarks: Vec<Landmark> = self.game.get_active_landmarks().to_vec();
    for landmark in active_landmarks.iter() {
      LandmarkRules::on_dice_roll(
        *landmark,
        &mut self.game,
        dice_roll,
        &mut *self.player_strategies[current_player],
      );
    }

    dice_roll.0 + dice_roll.1
  }

  /// Phase 2: Earn income (activate cards)
  /// All cards are activated in the order of their color.
  /// Activation order: Red -> Blue and Green -> Purple -> Orange/Landmarks
  /// For red cards, pay coins in reverse order of players
  pub fn earn_income_phase(&mut self, dice_roll_sum: u8) {
    // Collect cards to activate (to avoid borrow conflicts). Vec<(card, card owner index)>
    // Pre-allocate with capacity for ~4 players * ~15 cards = 60 max
    let mut cards_to_activate: Vec<(Card, usize)> = Vec::with_capacity(60);

    // Collect red cards (all players except the current player)
    for player_index in self.game.other_players_reverse() {
      for card in self.game.players[player_index].cards.iter() {
        if card.def().color == CardColor::Red && card.def().activation.contains(&dice_roll_sum) {
          cards_to_activate.push((*card, player_index));
        }
      }
    }

    // Collect blue cards (all players)
    for (player_index, player) in self.game.players.iter().enumerate() {
      for card in player.cards.iter() {
        if card.def().color == CardColor::Blue && card.def().activation.contains(&dice_roll_sum) {
          cards_to_activate.push((*card, player_index));
        }
      }
    }

    // Collect green cards (current player)
    for card in self.game.players[self.game.current_player].cards.iter() {
      if card.def().color == CardColor::Green && card.def().activation.contains(&dice_roll_sum) {
        cards_to_activate.push((*card, self.game.current_player));
      }
    }

    // Collect purple cards (current player)
    for card in self.game.players[self.game.current_player].cards.iter() {
      if card.def().color == CardColor::Purple && card.def().activation.contains(&dice_roll_sum) {
        cards_to_activate.push((*card, self.game.current_player));
      }
    }

    // Activate collected cards
    let mut coins_received = false;
    for (card, player_index) in cards_to_activate {
      let coins_before = self.game.players[player_index].coins;
      activate_card(
        card,
        &mut self.game,
        player_index,
        &mut *self.player_strategies[player_index],
      );
      coins_received |= coins_before < self.game.players[player_index].coins;
    }
    LandmarkRules::on_after_card_activation(&mut self.game, coins_received);
  }

  /// Phase 3: Buy card or landmark
  /// If the player has no coins, get 1 coin from the bank. The player then also chooses to
  /// either buy a card or landmark, or do nothing.
  /// After the player has made their decision, the landmarks trigger their effects based on whether
  /// the player built something this turn.
  fn buy_phase(&mut self) {
    // If the player has no coins, get 1 coin from the bank
    if self.game.players[self.game.current_player].coins == 0 {
      self.game.get_coins_from_bank(self.game.current_player, 1);
    }

    let mut built_something_this_turn = false;

    let decision = self.player_strategies[self.game.current_player].decide_purchase(&self.game);
    debug_print_purchase_decision(decision);

    match decision {
      PurchaseDecision::BuyCard(card) => {
        self.game.buy_card(card);
        built_something_this_turn = true;
      }
      PurchaseDecision::BuyLandmark(landmark) => {
        self.game.buy_landmark(landmark);
        built_something_this_turn = true;

        // Activate built landmark
        LandmarkRules::activate_landmark(landmark, &mut self.game);
      }
      PurchaseDecision::BuyNothing => {}
    }

    LandmarkRules::on_turn_end(&mut self.game, built_something_this_turn);
  }
}
