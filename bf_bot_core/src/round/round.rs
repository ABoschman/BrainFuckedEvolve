#![allow(dead_code, unused_variables, unused_imports)]//TODO: Remove this debug line.

use bot::Bot;
use round::RoundParams;
use round::RoundResult;
use arena::Arena;

pub fn play(bot_a: &Bot, bot_b: &Bot, round_params: &RoundParams) -> RoundResult {
    Arena::new(bot_a, bot_b, round_params)
        .find(|&ref outcome| outcome.is_some())
        .unwrap()
        .unwrap()
}

//TODO: Merge arena into round.

// #[cfg(test)]
// #[allow(non_snake_case)]
// mod tests {
//     use super::*;
// }