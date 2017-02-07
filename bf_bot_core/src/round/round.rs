#![allow(dead_code, unused_variables, unused_imports)]//TODO: Remove this debug line.

use bot::Bot;
use round::round_params::RoundParams;
use round::round_result::RoundResult;
use arena::Arena;

pub fn play(bot_a: &Bot, bot_b: &Bot, round_params: &RoundParams) -> RoundResult {
    let mut arena = Arena::new(bot_a, bot_b, round_params.tape_length, round_params.invert_polarity);
    for i in 0..round_params.max_steps {
        arena.step();
        if arena.has_loser() { break; }
    }
    arena.generate_result()
}

// #[cfg(test)]
// #[allow(non_snake_case)]
// mod tests {
//     use super::*;
// }