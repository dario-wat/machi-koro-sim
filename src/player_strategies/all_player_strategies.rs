use rand::seq::SliceRandom;
use strum::{EnumIter, IntoEnumIterator};

use crate::player_strategies::{
  GreedyBestCardStrategy, LandmarkRushStrategy, OptimizedStrategy, PlayerStrategy, RandomStrategy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum PlayerStrategyType {
  Random,
  LandmarkRush,
  GreedyBestCard,
  Optimized,
}

pub fn get_player_strategy(strategy_type: PlayerStrategyType) -> Box<dyn PlayerStrategy> {
  match strategy_type {
    PlayerStrategyType::Random => Box::new(RandomStrategy::new()),
    PlayerStrategyType::LandmarkRush => Box::new(LandmarkRushStrategy::new()),
    PlayerStrategyType::GreedyBestCard => Box::new(GreedyBestCardStrategy::new()),
    PlayerStrategyType::Optimized => Box::new(OptimizedStrategy::new()),
  }
}

pub fn get_random_player_strategy() -> Box<dyn PlayerStrategy> {
  let mut rng = rand::thread_rng();
  let strategies = PlayerStrategyType::iter().collect::<Vec<PlayerStrategyType>>();
  let strategy = strategies.choose(&mut rng).unwrap();
  get_player_strategy(*strategy)
}
