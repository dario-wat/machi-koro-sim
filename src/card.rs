use std::fmt;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Card {
  pub name: &'static str,
  pub cost: u8,
  pub activation: &'static [u8],
  pub color: CardColor,
  pub category: CardCategory,
  // pub: effect
}

impl Card {
  pub const SUSHI_BAR: Card = Card {
    name: "Sushi Bar",
    cost: 2,
    activation: &[1],
    color: CardColor::Red,
    category: CardCategory::Cup,
    // take 3 coins from active player
  };
  pub const WHEAT_FIELD: Card = Card {
    name: "Wheat Field",
    cost: 1,
    activation: &[1, 2],
    color: CardColor::Blue,
    category: CardCategory::Wheat,
    // get 1 coin from the bank
  };
  pub const VINEYARD: Card = Card {
    name: "Vineyard",
    cost: 1,
    activation: &[1, 2],
    color: CardColor::Blue,
    category: CardCategory::Fruit,
    // get 2 coins from the bank
  };
  pub const BAKERY: Card = Card {
    name: "Bakery",
    cost: 1,
    activation: &[2, 3],
    color: CardColor::Green,
    category: CardCategory::Bread,
    // get 2 coins from the bank
  };
  pub const CAFE: Card = Card {
    name: "Cafe",
    cost: 1,
    activation: &[3],
    color: CardColor::Red,
    category: CardCategory::Cup,
    // take 2 coins from active player
  };
  pub const FLOWER_GARDEN: Card = Card {
    name: "Flower Garden",
    cost: 2,
    activation: &[4],
    color: CardColor::Blue,
    category: CardCategory::Flower,
    // get 2 coins from the bank
  };
  pub const CONVENIENCE_STORE: Card = Card {
    name: "Convenience Store",
    cost: 1,
    activation: &[4],
    color: CardColor::Green,
    category: CardCategory::Bread,
    // get 3 coins from the bank
  };
  pub const FOREST: Card = Card {
    name: "Forest",
    cost: 3,
    activation: &[5],
    color: CardColor::Blue,
    category: CardCategory::Gear,
    // get 2 coins from the bank
  };
  pub const CORN_FIELD: Card = Card {
    name: "Corn Field",
    cost: 2,
    activation: &[7],
    color: CardColor::Blue,
    category: CardCategory::Wheat,
    // get 3 coins from the bank
  };
  pub const HAMBURGER_STAND: Card = Card {
    name: "Hamburger Stand",
    cost: 1,
    activation: &[8],
    color: CardColor::Red,
    category: CardCategory::Cup,
    // take 2 coins from active player
  };
  pub const FAMILY_RESTAURANT: Card = Card {
    name: "Family Restaurant",
    cost: 2,
    activation: &[9, 10],
    color: CardColor::Red,
    category: CardCategory::Cup,
    // take 2 coins from active player
  };
  pub const APPLE_ORCHARD: Card = Card {
    name: "Apple Orchard",
    cost: 1,
    activation: &[10],
    color: CardColor::Blue,
    category: CardCategory::Fruit,
    // get 3 coins from the bank
  };
  pub const MINE: Card = Card {
    name: "Mine",
    cost: 4,
    activation: &[11, 12],
    color: CardColor::Blue,
    category: CardCategory::Gear,
    // get 6 coins from the bank
  };
  pub const FLOWER_SHOP: Card = Card {
    name: "Flower Shop",
    cost: 1,
    activation: &[6],
    color: CardColor::Green,
    category: CardCategory::Combo,
    // get 3 coins from bank for each flower card
  };
  pub const BUSINESS_CENTER: Card = Card {
    name: "Business Center",
    cost: 3,
    activation: &[6],
    color: CardColor::Purple,
    category: CardCategory::Building,
    // exhange 1 of your establishmen for one of opponents
  };
  pub const STADIUM: Card = Card {
    name: "Stadium",
    cost: 3,
    activation: &[7],
    color: CardColor::Purple,
    category: CardCategory::Building,
    // take 3 coins from each opponent
  };
  pub const FURNITURE_FACTORY: Card = Card {
    name: "Furniture Factory",
    cost: 4,
    activation: &[8],
    color: CardColor::Green,
    category: CardCategory::Combo,
    // get 4 coins for each gear card
  };
  pub const SHOPPING_DISTRICT: Card = Card {
    name: "Shopping District",
    cost: 3,
    activation: &[8, 9],
    color: CardColor::Purple,
    category: CardCategory::Building,
    // for each opponent who has more than 10 coins, take half rounded down
  };
  pub const WINERY: Card = Card {
    name: "Winery",
    cost: 3,
    activation: &[9],
    color: CardColor::Green,
    category: CardCategory::Combo,
    // get 3 coins for each fruit card
  };
  pub const FOOD_WAREHOUSE: Card = Card {
    name: "Food Warehouse",
    cost: 2,
    activation: &[10, 11],
    color: CardColor::Green,
    category: CardCategory::Combo,
    // get 2 coins for each cup card
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

pub fn build_less_than_6_deck() -> Vec<Card> {
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
