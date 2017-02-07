use bot::Bot;
use round;
use round::round_params::RoundParams;
use game::game_result::GameResult;
use game::all_rounds::AllRounds;

/// Compares two bots in a (complete) game and returns the result.
pub fn run_complete(bot_a: &Bot, bot_b: &Bot) -> GameResult {
    run(bot_a, bot_b, AllRounds::new())
}

/// Compares two bots in a game consisting of the provided rounds. Returns the result of the game.
pub fn run<I>(bot_a: &Bot, bot_b: &Bot, rounds: I) -> GameResult
    where I: Iterator<Item=RoundParams> {
    rounds.fold(GameResult::new(), | mut game_result, round_params | {
        let round_result = round::play(bot_a, bot_b, &round_params);
        game_result.add_result_to_total(&round_result);
        game_result
    })
}