use round::round_result::RoundResult;

/// The result of a game of Brainfuck joust. A game consists of multiple rounds.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct GameResult {
    pub bot_a_points: i8,
    pub bot_b_points: i8,
}

impl GameResult {
    pub fn new() -> GameResult {
        GameResult { bot_a_points: 0, bot_b_points: 0 }
    }

    pub fn add_result_to_total(&mut self, round_result: &RoundResult) {
        if round_result.has_winner() {
            self.bot_a_points += if round_result.bot_a_lost { -1 } else { 1 };
            self.bot_b_points += if round_result.bot_b_lost { -1 } else { 1 };
        }
    }
}