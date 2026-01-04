use std::fmt;

use strum::EnumIter;

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
pub enum CardEffect {
  TakeCoinsFromActivePlayer(u16),
  TakeCoinsFromEachOpponent(u16),
  TakeCoinsFromEachOpponentWithMoreThan10Coins,
  GetCoinsFromBank(u16),
  GetCoinsFromBankForEachCardCategory(u16, CardCategory),
  GetCoinsFromBankForEachCardColor(u16, CardColor),
  ExchangeEstablishment,
}

impl fmt::Display for CardEffect {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CardEffect::TakeCoinsFromActivePlayer(amount) => {
        write!(f, "Take {}C from active", amount)
      }
      CardEffect::TakeCoinsFromEachOpponent(amount) => {
        write!(f, "Take {}C from others", amount)
      }
      CardEffect::TakeCoinsFromEachOpponentWithMoreThan10Coins => {
        write!(f, "Take 1/2 from >10C")
      }
      CardEffect::GetCoinsFromBank(amount) => write!(f, "Get {}C", amount),
      CardEffect::GetCoinsFromBankForEachCardCategory(amount, category) => {
        write!(f, "{}C per {}", amount, category)
      }
      CardEffect::GetCoinsFromBankForEachCardColor(amount, color) => {
        write!(f, "{}C per {}", amount, color)
      }
      CardEffect::ExchangeEstablishment => write!(f, "Exchange est."),
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CardDef {
  pub name: &'static str,
  pub cost: u16,
  pub activation: &'static [u8],
  pub color: CardColor,
  pub category: CardCategory,
  pub effect: CardEffect,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumIter)]
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
        effect: CardEffect::TakeCoinsFromActivePlayer(3),
      },
      Card::WheatField => CardDef {
        name: "Wheat Field",
        cost: 1,
        activation: &[1, 2],
        color: CardColor::Blue,
        category: CardCategory::Wheat,
        effect: CardEffect::GetCoinsFromBank(1),
      },
      Card::Vineyard => CardDef {
        name: "Vineyard",
        cost: 1,
        activation: &[1, 2],
        color: CardColor::Blue,
        category: CardCategory::Fruit,
        effect: CardEffect::GetCoinsFromBank(2),
      },
      Card::Bakery => CardDef {
        name: "Bakery",
        cost: 1,
        activation: &[2, 3],
        color: CardColor::Green,
        category: CardCategory::Bread,
        effect: CardEffect::GetCoinsFromBank(2),
      },
      Card::Cafe => CardDef {
        name: "Cafe",
        cost: 1,
        activation: &[3],
        color: CardColor::Red,
        category: CardCategory::Cup,
        effect: CardEffect::TakeCoinsFromActivePlayer(2),
      },
      Card::FlowerGarden => CardDef {
        name: "Flower Garden",
        cost: 2,
        activation: &[4],
        color: CardColor::Blue,
        category: CardCategory::Flower,
        effect: CardEffect::GetCoinsFromBank(2),
      },
      Card::ConvenienceStore => CardDef {
        name: "Convenience Store",
        cost: 1,
        activation: &[4],
        color: CardColor::Green,
        category: CardCategory::Bread,
        effect: CardEffect::GetCoinsFromBank(3),
      },
      Card::Forest => CardDef {
        name: "Forest",
        cost: 3,
        activation: &[5],
        color: CardColor::Blue,
        category: CardCategory::Gear,
        effect: CardEffect::GetCoinsFromBank(2),
      },
      Card::CornField => CardDef {
        name: "Corn Field",
        cost: 2,
        activation: &[7],
        color: CardColor::Blue,
        category: CardCategory::Wheat,
        effect: CardEffect::GetCoinsFromBank(3),
      },
      Card::HamburgerStand => CardDef {
        name: "Hamburger Stand",
        cost: 1,
        activation: &[8],
        color: CardColor::Red,
        category: CardCategory::Cup,
        effect: CardEffect::TakeCoinsFromActivePlayer(2),
      },
      Card::FamilyRestaurant => CardDef {
        name: "Family Restaurant",
        cost: 2,
        activation: &[9, 10],
        color: CardColor::Red,
        category: CardCategory::Cup,
        effect: CardEffect::TakeCoinsFromActivePlayer(2),
      },
      Card::AppleOrchard => CardDef {
        name: "Apple Orchard",
        cost: 1,
        activation: &[10],
        color: CardColor::Blue,
        category: CardCategory::Fruit,
        effect: CardEffect::GetCoinsFromBank(3),
      },
      Card::Mine => CardDef {
        name: "Mine",
        cost: 4,
        activation: &[11, 12],
        color: CardColor::Blue,
        category: CardCategory::Gear,
        effect: CardEffect::GetCoinsFromBank(6),
      },
      Card::FlowerShop => CardDef {
        name: "Flower Shop",
        cost: 1,
        activation: &[6],
        color: CardColor::Green,
        category: CardCategory::Combo,
        effect: CardEffect::GetCoinsFromBankForEachCardCategory(3, CardCategory::Flower),
      },
      Card::BusinessCenter => CardDef {
        name: "Business Center",
        cost: 3,
        activation: &[6],
        color: CardColor::Purple,
        category: CardCategory::Building,
        effect: CardEffect::ExchangeEstablishment,
      },
      Card::Stadium => CardDef {
        name: "Stadium",
        cost: 3,
        activation: &[7],
        color: CardColor::Purple,
        category: CardCategory::Building,
        effect: CardEffect::TakeCoinsFromEachOpponent(3),
      },
      Card::FurnitureFactory => CardDef {
        name: "Furniture Factory",
        cost: 4,
        activation: &[8],
        color: CardColor::Green,
        category: CardCategory::Combo,
        effect: CardEffect::GetCoinsFromBankForEachCardColor(4, CardColor::Green),
      },
      Card::ShoppingDistrict => CardDef {
        name: "Shopping District",
        cost: 3,
        activation: &[8, 9],
        color: CardColor::Purple,
        category: CardCategory::Building,
        effect: CardEffect::TakeCoinsFromEachOpponentWithMoreThan10Coins,
      },
      Card::Winery => CardDef {
        name: "Winery",
        cost: 3,
        activation: &[9],
        color: CardColor::Green,
        category: CardCategory::Combo,
        effect: CardEffect::GetCoinsFromBankForEachCardCategory(3, CardCategory::Fruit),
      },
      Card::FoodWarehouse => CardDef {
        name: "Food Warehouse",
        cost: 2,
        activation: &[10, 11],
        color: CardColor::Green,
        category: CardCategory::Combo,
        effect: CardEffect::GetCoinsFromBankForEachCardCategory(2, CardCategory::Cup),
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
