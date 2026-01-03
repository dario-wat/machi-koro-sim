use std::fmt;

use strum::EnumIter;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum LandmarkType {
  Immediate,
  Infinite,
}

impl fmt::Display for LandmarkType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LandmarkType::Immediate => write!(f, "Immediate"),
      LandmarkType::Infinite => write!(f, "Infinite"),
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LandmarkDef {
  pub name: &'static str,
  pub cost: &'static [u16],
  pub landmark_type: LandmarkType,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumIter)]
pub enum Landmark {
  Airport,
  AmusementPark,
  Charterhouse,
  ExhibitHall,
  FarmersMarket,
  Forge,
  FrenchRestaurant,
  LaunchPad,
  LoanOffice,
  MovingCompany,
  Museum,
  Observatory,
  Park,
  Publisher,
  RadioTower,
  SodaBottlingPlant,
  ShoppingMall,
  TechStartup,
  Temple,
  TvStation,
}

impl Landmark {
  pub const fn def(&self) -> LandmarkDef {
    match self {
      Landmark::Airport => LandmarkDef {
        name: "Airport",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::AmusementPark => LandmarkDef {
        name: "Amusement Park",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::Charterhouse => LandmarkDef {
        name: "Charterhouse",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::ExhibitHall => LandmarkDef {
        name: "Exhibit Hall",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Immediate,
      },
      Landmark::FarmersMarket => LandmarkDef {
        name: "Farmers Market",
        cost: &[10, 14, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::Forge => LandmarkDef {
        name: "Forge",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::FrenchRestaurant => LandmarkDef {
        name: "French Restaurant",
        cost: &[10, 14, 22],
        landmark_type: LandmarkType::Immediate,
      },
      Landmark::LaunchPad => LandmarkDef {
        name: "Launch Pad",
        cost: &[45, 38, 25],
        landmark_type: LandmarkType::Immediate,
      },
      Landmark::LoanOffice => LandmarkDef {
        name: "Loan Office",
        cost: &[10],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::MovingCompany => LandmarkDef {
        name: "Moving Company",
        cost: &[10, 14, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::Museum => LandmarkDef {
        name: "Museum",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Immediate,
      },
      Landmark::Observatory => LandmarkDef {
        name: "Observatory",
        cost: &[10, 14, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::Park => LandmarkDef {
        name: "Park",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Immediate,
      },
      Landmark::Publisher => LandmarkDef {
        name: "Publisher",
        cost: &[10, 14, 22],
        landmark_type: LandmarkType::Immediate,
      },
      Landmark::RadioTower => LandmarkDef {
        name: "Radio Tower",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Immediate,
      },
      Landmark::SodaBottlingPlant => LandmarkDef {
        name: "Soda Bottling Plant",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::ShoppingMall => LandmarkDef {
        name: "Shopping Mall",
        cost: &[10, 14, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::TechStartup => LandmarkDef {
        name: "Tech Startup",
        cost: &[10, 14, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::Temple => LandmarkDef {
        name: "Temple",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Infinite,
      },
      Landmark::TvStation => LandmarkDef {
        name: "TV Station",
        cost: &[12, 16, 22],
        landmark_type: LandmarkType::Immediate,
      },
    }
  }
}

impl fmt::Display for LandmarkDef {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let cost_str = self
      .cost
      .iter()
      .map(|n| n.to_string())
      .collect::<Vec<_>>()
      .join(", ");
    writeln!(f, "{{")?;
    writeln!(f, "  name: \"{}\",", self.name)?;
    writeln!(f, "  cost: [{}],", cost_str)?;
    writeln!(f, "  landmark_type: \"{}\"", self.landmark_type)?;
    write!(f, "}}")
  }
}
