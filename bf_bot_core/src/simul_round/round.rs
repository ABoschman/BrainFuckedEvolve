use bf::Bot;
use simul_round::RoundParams;
use simul_round::RoundResult;
use engine::Arena;

pub fn play(bot_a: &Bot, bot_b: &Bot, round_params: &RoundParams) -> RoundResult {
    Arena::new(bot_a, bot_b, round_params)
        .find(|&ref outcome| outcome.has_loser())
        .unwrap()
}

//TODO: Merge arena into round.

// #[cfg(test)]
// #[allow(non_snake_case)]
// mod tests {
//     use super::*;
// }
