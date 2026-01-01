use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CardColor {
  Blue,
  Green,
  Purple,
  Red,
}

impl fmt::Display for CardColor {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CardColor::Blue => write!(f, "Blue"),
      CardColor::Green => write!(f, "Green"),
      CardColor::Purple => write!(f, "Purple"),
      CardColor::Red => write!(f, "Red"),
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CardCategory {
  Bread,
  Building,
  Combo,
  Cup,
  Flower,
  Fruit,
  Gear,
  Wheat,
}

impl fmt::Display for CardCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CardCategory::Bread => write!(f, "Bread"),
      CardCategory::Building => write!(f, "Building"),
      CardCategory::Combo => write!(f, "Combo"),
      CardCategory::Cup => write!(f, "Cup"),
      CardCategory::Flower => write!(f, "Flower"),
      CardCategory::Fruit => write!(f, "Fruit"),
      CardCategory::Gear => write!(f, "Gear"),
      CardCategory::Wheat => write!(f, "Wheat"),
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Card {
  pub name: &'static str,
  pub cost: u8,
  pub activation: &'static [u8],
  pub color: CardColor,
  pub category: CardCategory,
  pub apply_effect: fn(&mut crate::game::Game, owner_index: usize),
}

/// Helper function to give coins from the bank to a player
fn get_coins_from_bank(game: &mut crate::game::Game, owner_index: usize, amount: u8) {
  game.players[owner_index].coins += amount;
}

/// Helper function to move coins between two players
fn move_coins_between_players(
  game: &mut crate::game::Game,
  from_index: usize,
  to_index: usize,
  amount: u8,
) {
  let from_coins = game.players[from_index].coins;
  let coins_to_move = std::cmp::min(from_coins, amount);
  game.players[from_index].coins -= coins_to_move;
  game.players[to_index].coins += coins_to_move;
}

/// Helper function to take coins from active player and give to owner
fn take_coins_from_active_player(game: &mut crate::game::Game, owner_index: usize, amount: u8) {
  move_coins_between_players(game, game.current_player as usize, owner_index, amount);
}

/// Helper function to get coins from bank for each card that matches the predicate
fn get_coins_from_bank_for_each_card(
  game: &mut crate::game::Game,
  owner_index: usize,
  amount: u8,
  predicate: fn(&Card) -> bool,
) {
  let coins_to_get = game.players[owner_index]
    .cards
    .iter()
    .filter(|card| predicate(card))
    .map(|card| amount)
    .sum();
  get_coins_from_bank(game, owner_index, coins_to_get);
}

impl Card {
  pub const SUSHI_BAR: Card = Card {
    name: "Sushi Bar",
    cost: 2,
    activation: &[1],
    color: CardColor::Red,
    category: CardCategory::Cup,
    apply_effect: |game, owner_index| {
      take_coins_from_active_player(game, owner_index, 3);
    },
  };
  pub const WHEAT_FIELD: Card = Card {
    name: "Wheat Field",
    cost: 1,
    activation: &[1, 2],
    color: CardColor::Blue,
    category: CardCategory::Wheat,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 1);
    },
  };
  pub const VINEYARD: Card = Card {
    name: "Vineyard",
    cost: 1,
    activation: &[1, 2],
    color: CardColor::Blue,
    category: CardCategory::Fruit,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 2);
    },
  };
  pub const BAKERY: Card = Card {
    name: "Bakery",
    cost: 1,
    activation: &[2, 3],
    color: CardColor::Green,
    category: CardCategory::Bread,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 2);
    },
  };
  pub const CAFE: Card = Card {
    name: "Cafe",
    cost: 1,
    activation: &[3],
    color: CardColor::Red,
    category: CardCategory::Cup,
    apply_effect: |game, owner_index| {
      take_coins_from_active_player(game, owner_index, 2);
    },
  };
  pub const FLOWER_GARDEN: Card = Card {
    name: "Flower Garden",
    cost: 2,
    activation: &[4],
    color: CardColor::Blue,
    category: CardCategory::Flower,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 2);
    },
  };
  pub const CONVENIENCE_STORE: Card = Card {
    name: "Convenience Store",
    cost: 1,
    activation: &[4],
    color: CardColor::Green,
    category: CardCategory::Bread,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 3);
    },
  };
  pub const FOREST: Card = Card {
    name: "Forest",
    cost: 3,
    activation: &[5],
    color: CardColor::Blue,
    category: CardCategory::Gear,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 2);
    },
  };
  pub const CORN_FIELD: Card = Card {
    name: "Corn Field",
    cost: 2,
    activation: &[7],
    color: CardColor::Blue,
    category: CardCategory::Wheat,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 3);
    },
  };
  pub const HAMBURGER_STAND: Card = Card {
    name: "Hamburger Stand",
    cost: 1,
    activation: &[8],
    color: CardColor::Red,
    category: CardCategory::Cup,
    apply_effect: |game, owner_index| {
      take_coins_from_active_player(game, owner_index, 2);
    },
  };
  pub const FAMILY_RESTAURANT: Card = Card {
    name: "Family Restaurant",
    cost: 2,
    activation: &[9, 10],
    color: CardColor::Red,
    category: CardCategory::Cup,
    apply_effect: |game, owner_index| {
      take_coins_from_active_player(game, owner_index, 2);
    },
  };
  pub const APPLE_ORCHARD: Card = Card {
    name: "Apple Orchard",
    cost: 1,
    activation: &[10],
    color: CardColor::Blue,
    category: CardCategory::Fruit,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 3);
    },
  };
  pub const MINE: Card = Card {
    name: "Mine",
    cost: 4,
    activation: &[11, 12],
    color: CardColor::Blue,
    category: CardCategory::Gear,
    apply_effect: |game, owner_index| {
      get_coins_from_bank(game, owner_index, 6);
    },
  };
  pub const FLOWER_SHOP: Card = Card {
    name: "Flower Shop",
    cost: 1,
    activation: &[6],
    color: CardColor::Green,
    category: CardCategory::Combo,
    apply_effect: |game, owner_index| {
      get_coins_from_bank_for_each_card(game, owner_index, 3, |card| {
        card.category == CardCategory::Flower
      });
    },
  };
  pub const BUSINESS_CENTER: Card = Card {
    name: "Business Center",
    cost: 3,
    activation: &[6],
    color: CardColor::Purple,
    category: CardCategory::Building,
    // TODO exhange 1 of your establishmen for one of opponents
    apply_effect: |game, owner_index| {},
  };
  pub const STADIUM: Card = Card {
    name: "Stadium",
    cost: 3,
    activation: &[7],
    color: CardColor::Purple,
    category: CardCategory::Building,
    apply_effect: |game, owner_index| {
      for player_index in 0..game.players.len() {
        if player_index != owner_index {
          move_coins_between_players(game, game.current_player as usize, player_index, 3);
        }
      }
    },
  };
  pub const FURNITURE_FACTORY: Card = Card {
    name: "Furniture Factory",
    cost: 4,
    activation: &[8],
    color: CardColor::Green,
    category: CardCategory::Combo,
    apply_effect: |game, owner_index| {
      get_coins_from_bank_for_each_card(game, owner_index, 4, |card| {
        card.color == CardColor::Green
      });
    },
  };
  pub const SHOPPING_DISTRICT: Card = Card {
    name: "Shopping District",
    cost: 3,
    activation: &[8, 9],
    color: CardColor::Purple,
    category: CardCategory::Building,
    apply_effect: |game, owner_index| {
      for player_index in 0..game.players.len() {
        if player_index != owner_index && game.players[player_index].coins > 10 {
          let coins_to_take = game.players[player_index].coins / 2;
          move_coins_between_players(game, player_index, owner_index, coins_to_take);
        }
      }
    },
  };
  pub const WINERY: Card = Card {
    name: "Winery",
    cost: 3,
    activation: &[9],
    color: CardColor::Green,
    category: CardCategory::Combo,
    apply_effect: |game, owner_index| {
      get_coins_from_bank_for_each_card(game, owner_index, 3, |card| {
        card.category == CardCategory::Fruit
      });
    },
  };
  pub const FOOD_WAREHOUSE: Card = Card {
    name: "Food Warehouse",
    cost: 2,
    activation: &[10, 11],
    color: CardColor::Green,
    category: CardCategory::Combo,
    apply_effect: |game, owner_index| {
      get_coins_from_bank_for_each_card(game, owner_index, 2, |card| {
        card.category == CardCategory::Cup
      });
    },
  };
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let activation_str = self
      .activation
      .iter()
      .map(|n| n.to_string())
      .collect::<Vec<_>>()
      .join(", ");
    writeln!(f, "{{")?;
    writeln!(f, "  name: \"{}\",", self.name)?;
    writeln!(f, "  cost: {},", self.cost)?;
    writeln!(f, "  activation: [{}],", activation_str)?;
    writeln!(f, "  color: \"{}\",", self.color)?;
    writeln!(f, "  category: \"{}\"", self.category)?;
    write!(f, "}}")
  }
}

pub struct DeckEntry {
  pub card: Card,
  pub copies: u8,
}

pub const DECK_COMPOSITION: &[DeckEntry] = &[
  DeckEntry {
    card: Card::SUSHI_BAR,
    copies: 5,
  },
  DeckEntry {
    card: Card::WHEAT_FIELD,
    copies: 5,
  },
  DeckEntry {
    card: Card::VINEYARD,
    copies: 5,
  },
  DeckEntry {
    card: Card::BAKERY,
    copies: 5,
  },
  DeckEntry {
    card: Card::CAFE,
    copies: 5,
  },
  DeckEntry {
    card: Card::FLOWER_GARDEN,
    copies: 5,
  },
  DeckEntry {
    card: Card::CONVENIENCE_STORE,
    copies: 5,
  },
  DeckEntry {
    card: Card::FOREST,
    copies: 5,
  },
  DeckEntry {
    card: Card::CORN_FIELD,
    copies: 5,
  },
  DeckEntry {
    card: Card::HAMBURGER_STAND,
    copies: 5,
  },
  DeckEntry {
    card: Card::FAMILY_RESTAURANT,
    copies: 5,
  },
  DeckEntry {
    card: Card::APPLE_ORCHARD,
    copies: 5,
  },
  DeckEntry {
    card: Card::MINE,
    copies: 5,
  },
  DeckEntry {
    card: Card::FLOWER_SHOP,
    copies: 3,
  },
  DeckEntry {
    card: Card::BUSINESS_CENTER,
    copies: 3,
  },
  DeckEntry {
    card: Card::STADIUM,
    copies: 3,
  },
  DeckEntry {
    card: Card::FURNITURE_FACTORY,
    copies: 3,
  },
  DeckEntry {
    card: Card::SHOPPING_DISTRICT,
    copies: 3,
  },
  DeckEntry {
    card: Card::WINERY,
    copies: 3,
  },
  DeckEntry {
    card: Card::FOOD_WAREHOUSE,
    copies: 3,
  },
];

pub fn build_less_than_7_deck() -> Vec<Card> {
  DECK_COMPOSITION
    .iter()
    .filter(|entry| {
      entry
        .card
        .activation
        .iter()
        .all(|&activation| activation <= 6)
        && entry.card.cost <= 6
    })
    .flat_map(|entry| std::iter::repeat(entry.card.clone()).take(entry.copies.into()))
    .collect()
}

pub fn build_greater_than_6_deck() -> Vec<Card> {
  DECK_COMPOSITION
    .iter()
    .filter(|entry| {
      entry
        .card
        .activation
        .iter()
        .all(|&activation| activation > 6)
    })
    .flat_map(|entry| std::iter::repeat(entry.card.clone()).take(entry.copies.into()))
    .collect()
}
