pub enum LandmarkType {
  Immediate,
  Infinite,
}

pub struct Landmark {
  pub name: &'static str,
  pub cost: &'static [u8],
  pub landmarkType: LandmarkType,
}

impl Landmark {
  pub const AIRPORT: Landmark = Landmark {
    name: "Airport",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Infinite,
    // if you dont build anything on your turn, get 5 coins from the bank
  };
  pub const AMUSEMENT_PARK: Landmark = Landmark {
    name: "Amusement Park",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Infinite,
    // if you roll doubles, take another turn after this one
  };
  pub const CHARTERHOUSE: Landmark = Landmark {
    name: "Charterhouse",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Infinite,
    // if you roll two dice and receive no coins, get 3 coins from the bank
  };
  pub const EXHIBIT_HALL: Landmark = Landmark {
    name: "Exhibit Hall",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Immediate,
    // from each opponent with more than 10 coins, take half, rounded down
  };
  pub const FARMERS_MARKET: Landmark = Landmark {
    name: "Farmers Market",
    cost: &[10, 14, 22],
    landmarkType: LandmarkType::Infinite,
    // wheat establishments earn +1 coin
  };
  pub const FORGE: Landmark = Landmark {
    name: "Forge",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Infinite,
    // gear establishments earn +1 coin
  };
  pub const FRENCH_RESTAURANT: Landmark = Landmark {
    name: "French Restaurant",
    cost: &[10, 14, 22],
    landmarkType: LandmarkType::Immediate,
    // take 2 coins from each opponent
  };
  pub const LAUNCH_PAD: Landmark = Landmark {
    name: "Launch Pad",
    cost: &[45, 38, 25],
    landmarkType: LandmarkType::Immediate,
    // you win
  };
  pub const LOAN_OFFICE: Landmark = Landmark {
    name: "Loan Office",
    cost: &[10],
    landmarkType: LandmarkType::Infinite,
    // you can buy only when the only player withou landmark. reduce build
    // cost of landmarks by 2 coins
  };
  pub const MOVING_COMPANY: Landmark = Landmark {
    name: "Moving Company",
    cost: &[10, 14, 22],
    landmarkType: LandmarkType::Infinite,
    // if you roll doubles, give 1 of your establishments to the player on the right
  };
  pub const MUSEUM: Landmark = Landmark {
    name: "Museum",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Immediate,
    // take 3 coins from each opponent for each landmark they own
  };
  pub const OBSERVATORY: Landmark = Landmark {
    name: "Observatory",
    cost: &[10, 14, 22],
    landmarkType: LandmarkType::Infinite,
    // reduce build cost of launch pad by 5 coins
  };
  pub const PARK: Landmark = Landmark {
    name: "Park",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Immediate,
    // redistribute coins evenly, making up with coins from the bank
  };
  pub const PUBLISHER: Landmark = Landmark {
    name: "Publisher",
    cost: &[10, 14, 22],
    landmarkType: LandmarkType::Immediate,
    // take 1 coin from each opponent for each bread card
  };
  pub const RADIO_TOWER: Landmark = Landmark {
    name: "Radio Tower",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Immediate,
    // Take another turn after this one
  };
  pub const SODA_BOTTLING_PLANT: Landmark = Landmark {
    name: "Soda Bottling Plant",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Infinite,
    // your cup cards earn +1 coin when activated
  };
  pub const SHOPPING_MALL: Landmark = Landmark {
    name: "Shopping Mall",
    cost: &[10, 14, 22],
    landmarkType: LandmarkType::Infinite,
    // your bread cards earn +1 coin when activated
  };
  pub const TECH_STARTUP: Landmark = Landmark {
    name: "Tech Startup",
    cost: &[10, 14, 22],
    landmarkType: LandmarkType::Infinite,
    // if you roll 12, get 8 coins from the bank
  };
  pub const TEMPLE: Landmark = Landmark {
    name: "Temple",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Infinite,
    // if you roll doubles, take 2 coins from each opponent
  };
  pub const TV_STATION: Landmark = Landmark {
    name: "TV Station",
    cost: &[12, 16, 22],
    landmarkType: LandmarkType::Immediate,
    // take 1 coin from each opponent for each cup card
  };
}
