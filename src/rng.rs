use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng as RandRng, SeedableRng};

pub struct Rng {
  rng: StdRng,
}

impl Rng {
  /// Create a new RNG with a seed
  pub fn new_with_seed(seed: u64) -> Self {
    Self {
      rng: StdRng::seed_from_u64(seed),
    }
  }

  /// Create a new RNG with a random seed
  pub fn new() -> Self {
    let seed = rand::thread_rng().gen::<u64>();
    Self {
      rng: StdRng::seed_from_u64(seed),
    }
  }

  /// Roll a single die (returns 1-6)
  pub fn roll_die(&mut self) -> u8 {
    RandRng::gen_range(&mut self.rng, 1u8..=6u8)
  }

  /// Roll two dice (returns 2-12)
  pub fn roll_two_dice(&mut self) -> u8 {
    self.roll_die() + self.roll_die()
  }

  /// Shuffle a mutable slice in place using the RNG seed
  pub fn shuffle<T>(&mut self, slice: &mut [T]) {
    slice.shuffle(&mut self.rng);
  }
}
