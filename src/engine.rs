use crate::{
  game::{Game, BUY_ONLY_TURNS, MAX_PLAYERS, MIN_PLAYERS},
  game_debug::debug_print,
  models::{Card, CardColor, Player},
  player_strategies::{
    player_strategy::{DiceRollDecision, PurchaseDecision},
    PlayerStrategy,
  },
  rules::card::activate_card,
};

pub struct Engine {
  game: Game,
  player_strategies: Vec<Box<dyn PlayerStrategy>>,
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

    debug_print(&self.game);
    println!("--------------------------------");
    while self.game.winner().is_none() {
      self.play_turn();
      debug_print(&self.game);
      println!("--------------------------------");
    }
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
    let decision = self.player_strategies[self.game.current_player].decide_purchase(&self.game);
    println!("{}", decision);
    match decision {
      PurchaseDecision::BuyCard(card) => self.game.buy_card(card),
      PurchaseDecision::BuyLandmark(landmark) => self.game.buy_landmark(landmark),
      PurchaseDecision::BuyNothing => {}
    }
  }

  /// Normal turn has 3 phases: Roll dice, earn income, buy card or landmark
  fn play_normal_turn(&mut self) {
    // Phase 1: Roll dice
    let decision = self.player_strategies[self.game.current_player].decide_dice_roll(&self.game);
    let dice_roll = match decision {
      DiceRollDecision::RollOne => (self.game.roll_one_die(), 0),
      DiceRollDecision::RollTwo => self.game.roll_two_dice(),
    };
    let dice_roll_sum = dice_roll.0 + dice_roll.1;
    if dice_roll.1 == 0 {
      println!("Dice roll: {} (total: {})", dice_roll.0, dice_roll_sum);
    } else {
      println!(
        "Dice roll: {} + {} (total: {})",
        dice_roll.0, dice_roll.1, dice_roll_sum
      );
    }
    // Phase 2: Earn income (activate cards)
    // Activation order: Red -> Blue and Green -> Purple -> Orange/Landmarks
    // Pay coins in reverse order of players

    // Collect cards to activate (to avoid borrow conflicts)
    let mut cards_to_activate: Vec<(Card, usize)> = Vec::new();
    for player_index in self.game.other_players_reverse() {
      for card in self.game.players[player_index].cards.iter() {
        if card.def().color == CardColor::Red {
          cards_to_activate.push((*card, player_index));
        }
      }
    }

    // Activate collected cards
    for (card, player_index) in cards_to_activate {
      activate_card(card, &mut self.game, player_index);
    }

    // Phase 3: Buy card or landmark
    // Receive 1 coin at the beginning of this phase if no coins
  }
}
