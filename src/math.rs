use crate::models::Card;

const TWO_DICE_PROBABILITY_TABLE: [f64; 11] = [
  1.0 / 36.0, // 2
  2.0 / 36.0, // 3
  3.0 / 36.0, // 4
  4.0 / 36.0, // 5
  5.0 / 36.0, // 6
  6.0 / 36.0, // 7
  5.0 / 36.0, // 8
  4.0 / 36.0, // 9
  3.0 / 36.0, // 10
  2.0 / 36.0, // 11
  1.0 / 36.0, // 12
];

/// Probability that a card is activated given a single die roll
fn p_card_activation_single_dice(card: Card) -> f64 {
  let activation_count = card.def().activation.iter().filter(|&a| *a <= 6).count();
  activation_count as f64 / 6.0
}
/// Probability that a card is activated given a double die roll
fn p_card_activation_double_dice(card: Card) -> f64 {
  card
    .def()
    .activation
    .iter()
    .filter(|&a| *a > 1)
    .map(|a| TWO_DICE_PROBABILITY_TABLE[(*a - 2) as usize])
    .sum()
}

/// Probability that a card is activated given an equal probability of rolling 1 or 2 dice
pub fn p_card_activation(card: Card) -> f64 {
  0.5 * p_card_activation_single_dice(card) + 0.5 * p_card_activation_double_dice(card)
}
