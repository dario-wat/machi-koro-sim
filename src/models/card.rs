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
pub struct CardDef {
  pub name: &'static str,
  pub cost: u8,
  pub activation: &'static [u8],
  pub color: CardColor,
  pub category: CardCategory,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Card {
  SushiBar,
  WheatField,
  Vineyard,
  Bakery,
  Cafe,
  FlowerGarden,
  ConvenienceStore,
  Forest,
  CornField,
  HamburgerStand,
  FamilyRestaurant,
  AppleOrchard,
  Mine,
  FlowerShop,
  BusinessCenter,
  Stadium,
  FurnitureFactory,
  ShoppingDistrict,
  Winery,
  FoodWarehouse,
}

impl Card {
  pub const fn def(&self) -> CardDef {
    match self {
      Card::SushiBar => CardDef {
        name: "Sushi Bar",
        cost: 2,
        activation: &[1],
        color: CardColor::Red,
        category: CardCategory::Cup,
      },
      Card::WheatField => CardDef {
        name: "Wheat Field",
        cost: 1,
        activation: &[1, 2],
        color: CardColor::Blue,
        category: CardCategory::Wheat,
      },
      Card::Vineyard => CardDef {
        name: "Vineyard",
        cost: 1,
        activation: &[1, 2],
        color: CardColor::Blue,
        category: CardCategory::Fruit,
      },
      Card::Bakery => CardDef {
        name: "Bakery",
        cost: 1,
        activation: &[2, 3],
        color: CardColor::Green,
        category: CardCategory::Bread,
      },
      Card::Cafe => CardDef {
        name: "Cafe",
        cost: 1,
        activation: &[3],
        color: CardColor::Red,
        category: CardCategory::Cup,
      },
      Card::FlowerGarden => CardDef {
        name: "Flower Garden",
        cost: 2,
        activation: &[4],
        color: CardColor::Blue,
        category: CardCategory::Flower,
      },
      Card::ConvenienceStore => CardDef {
        name: "Convenience Store",
        cost: 1,
        activation: &[4],
        color: CardColor::Green,
        category: CardCategory::Bread,
      },
      Card::Forest => CardDef {
        name: "Forest",
        cost: 3,
        activation: &[5],
        color: CardColor::Blue,
        category: CardCategory::Gear,
      },
      Card::CornField => CardDef {
        name: "Corn Field",
        cost: 2,
        activation: &[7],
        color: CardColor::Blue,
        category: CardCategory::Wheat,
      },
      Card::HamburgerStand => CardDef {
        name: "Hamburger Stand",
        cost: 1,
        activation: &[8],
        color: CardColor::Red,
        category: CardCategory::Cup,
      },
      Card::FamilyRestaurant => CardDef {
        name: "Family Restaurant",
        cost: 2,
        activation: &[9, 10],
        color: CardColor::Red,
        category: CardCategory::Cup,
      },
      Card::AppleOrchard => CardDef {
        name: "Apple Orchard",
        cost: 1,
        activation: &[10],
        color: CardColor::Blue,
        category: CardCategory::Fruit,
      },
      Card::Mine => CardDef {
        name: "Mine",
        cost: 4,
        activation: &[11, 12],
        color: CardColor::Blue,
        category: CardCategory::Gear,
      },
      Card::FlowerShop => CardDef {
        name: "Flower Shop",
        cost: 1,
        activation: &[6],
        color: CardColor::Green,
        category: CardCategory::Combo,
      },
      Card::BusinessCenter => CardDef {
        name: "Business Center",
        cost: 3,
        activation: &[6],
        color: CardColor::Purple,
        category: CardCategory::Building,
      },
      Card::Stadium => CardDef {
        name: "Stadium",
        cost: 3,
        activation: &[7],
        color: CardColor::Purple,
        category: CardCategory::Building,
      },
      Card::FurnitureFactory => CardDef {
        name: "Furniture Factory",
        cost: 4,
        activation: &[8],
        color: CardColor::Green,
        category: CardCategory::Combo,
      },
      Card::ShoppingDistrict => CardDef {
        name: "Shopping District",
        cost: 3,
        activation: &[8, 9],
        color: CardColor::Purple,
        category: CardCategory::Building,
      },
      Card::Winery => CardDef {
        name: "Winery",
        cost: 3,
        activation: &[9],
        color: CardColor::Green,
        category: CardCategory::Combo,
      },
      Card::FoodWarehouse => CardDef {
        name: "Food Warehouse",
        cost: 2,
        activation: &[10, 11],
        color: CardColor::Green,
        category: CardCategory::Combo,
      },
    }
  }
}

impl fmt::Display for CardDef {
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

struct DeckEntry {
  pub card: Card,
  pub copies: u8,
}

const DECK_COMPOSITION: &[DeckEntry] = &[
  DeckEntry {
    card: Card::SushiBar,
    copies: 5,
  },
  DeckEntry {
    card: Card::WheatField,
    copies: 5,
  },
  DeckEntry {
    card: Card::Vineyard,
    copies: 5,
  },
  DeckEntry {
    card: Card::Bakery,
    copies: 5,
  },
  DeckEntry {
    card: Card::Cafe,
    copies: 5,
  },
  DeckEntry {
    card: Card::FlowerGarden,
    copies: 5,
  },
  DeckEntry {
    card: Card::ConvenienceStore,
    copies: 5,
  },
  DeckEntry {
    card: Card::Forest,
    copies: 5,
  },
  DeckEntry {
    card: Card::CornField,
    copies: 5,
  },
  DeckEntry {
    card: Card::HamburgerStand,
    copies: 5,
  },
  DeckEntry {
    card: Card::FamilyRestaurant,
    copies: 5,
  },
  DeckEntry {
    card: Card::AppleOrchard,
    copies: 5,
  },
  DeckEntry {
    card: Card::Mine,
    copies: 5,
  },
  DeckEntry {
    card: Card::FlowerShop,
    copies: 3,
  },
  DeckEntry {
    card: Card::BusinessCenter,
    copies: 3,
  },
  DeckEntry {
    card: Card::Stadium,
    copies: 3,
  },
  DeckEntry {
    card: Card::FurnitureFactory,
    copies: 3,
  },
  DeckEntry {
    card: Card::ShoppingDistrict,
    copies: 3,
  },
  DeckEntry {
    card: Card::Winery,
    copies: 3,
  },
  DeckEntry {
    card: Card::FoodWarehouse,
    copies: 3,
  },
];

pub fn build_less_than_7_deck() -> Vec<Card> {
  DECK_COMPOSITION
    .iter()
    .filter(|entry| {
      let def = entry.card.def();
      def.activation.iter().all(|&activation| activation <= 6)
    })
    .flat_map(|entry| std::iter::repeat(entry.card.clone()).take(entry.copies.into()))
    .collect()
}

pub fn build_greater_than_6_deck() -> Vec<Card> {
  DECK_COMPOSITION
    .iter()
    .filter(|entry| {
      let def = entry.card.def();
      def.activation.iter().all(|&activation| activation > 6)
    })
    .flat_map(|entry| std::iter::repeat(entry.card.clone()).take(entry.copies.into()))
    .collect()
}
