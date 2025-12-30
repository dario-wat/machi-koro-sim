pub enum CardColor {
  Blue,
  Green,
  Purple,
  Red,
}

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

pub struct Card {
  pub name: &'static str,
  pub cost: u8,
  pub activation: &'static [u8],
  pub color: CardColor,
  pub category: CardCategory,
  pub copies: u8,
  // pub: effect
}

impl Card {
  pub const SUSHI_BAR: Card = Card {
    name: "Sushi Bar",
    cost: 2,
    activation: &[1],
    color: CardColor::Red,
    category: CardCategory::Cup,
    copies: 5,
    // take 3 coins from active player
  };
  pub const WHEAT_FIELD: Card = Card {
    name: "Wheat Field",
    cost: 1,
    activation: &[1, 2],
    color: CardColor::Blue,
    category: CardCategory::Wheat,
    copies: 5,
    // get 1 coin from the bank
  };
  pub const VINEYARD: Card = Card {
    name: "Vineyard",
    cost: 1,
    activation: &[1, 2],
    color: CardColor::Blue,
    category: CardCategory::Fruit,
    copies: 5,
    // get 2 coins from the bank
  };
  pub const BAKERY: Card = Card {
    name: "Bakery",
    cost: 1,
    activation: &[2, 3],
    color: CardColor::Green,
    category: CardCategory::Bread,
    copies: 5,
    // get 2 coins from the bank
  };
  pub const CAFE: Card = Card {
    name: "Cafe",
    cost: 1,
    activation: &[3],
    color: CardColor::Red,
    category: CardCategory::Cup,
    copies: 5,
    // take 2 coins from active player
  };
  pub const FLOWER_GARDEN: Card = Card {
    name: "Flower Garden",
    cost: 2,
    activation: &[4],
    color: CardColor::Blue,
    category: CardCategory::Flower,
    copies: 5,
    // get 2 coins from the bank
  };
  pub const CONVENIENCE_STORE: Card = Card {
    name: "Convenience Store",
    cost: 1,
    activation: &[4],
    color: CardColor::Green,
    category: CardCategory::Bread,
    copies: 5,
    // get 3 coins from the bank
  };
  pub const FOREST: Card = Card {
    name: "Forest",
    cost: 3,
    activation: &[5],
    color: CardColor::Blue,
    category: CardCategory::Gear,
    copies: 5,
    // get 2 coins from the bank
  };
  pub const CORN_FIELD: Card = Card {
    name: "Corn Field",
    cost: 2,
    activation: &[7],
    color: CardColor::Blue,
    category: CardCategory::Wheat,
    copies: 5,
    // get 3 coins from the bank
  };
  pub const HAMBURGER_STAND: Card = Card {
    name: "Hamburger Stand",
    cost: 1,
    activation: &[8],
    color: CardColor::Red,
    category: CardCategory::Cup,
    copies: 5,
    // take 2 coins from active player
  };
  pub const FAMILY_RESTAURANT: Card = Card {
    name: "Family Restaurant",
    cost: 2,
    activation: &[9, 10],
    color: CardColor::Red,
    category: CardCategory::Cup,
    copies: 5,
    // take 2 coins from active player
  };
  pub const APPLE_ORCHARD: Card = Card {
    name: "Apple Orchard",
    cost: 1,
    activation: &[10],
    color: CardColor::Blue,
    category: CardCategory::Fruit,
    copies: 5,
    // get 3 coins from the bank
  };
  pub const MINE: Card = Card {
    name: "Mine",
    cost: 4,
    activation: &[11, 12],
    color: CardColor::Blue,
    category: CardCategory::Gear,
    copies: 5,
    // get 6 coins from the bank
  };
  pub const FLOWER_SHOP: Card = Card {
    name: "Flower Shop",
    cost: 1,
    activation: &[6],
    color: CardColor::Green,
    category: CardCategory::Combo,
    copies: 3,
    // get 3 coins from bank for each flower card
  };
  pub const BUSINESS_CENTER: Card = Card {
    name: "Business Center",
    cost: 3,
    activation: &[6],
    color: CardColor::Purple,
    category: CardCategory::Building,
    copies: 3,
    // exhange 1 of your establishmen for one of opponents
  };
  pub const STADIUM: Card = Card {
    name: "Stadium",
    cost: 3,
    activation: &[7],
    color: CardColor::Purple,
    category: CardCategory::Building,
    copies: 3,
    // take 3 coins from each opponent
  };
  pub const FURNITURE_FACTORY: Card = Card {
    name: "Furniture Factory",
    cost: 4,
    activation: &[8],
    color: CardColor::Green,
    category: CardCategory::Combo,
    copies: 3,
    // get 4 coins for each gear card
  };
  pub const SHOPPING_DISTRICT: Card = Card {
    name: "Shopping District",
    cost: 3,
    activation: &[8, 9],
    color: CardColor::Purple,
    category: CardCategory::Building,
    copies: 3,
    // for each opponent who has more than 10 coins, take half rounded down
  };
  pub const WINERY: Card = Card {
    name: "Winery",
    cost: 3,
    activation: &[9],
    color: CardColor::Green,
    category: CardCategory::Combo,
    copies: 3,
    // get 3 coins for each fruit card
  };
  pub const FOOD_WAREHOUSE: Card = Card {
    name: "Food Warehouse",
    cost: 2,
    activation: &[10, 11],
    color: CardColor::Green,
    category: CardCategory::Combo,
    copies: 3,
    // get 2 coins for each cup card
  };
}
