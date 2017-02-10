use round::RoundResult;

/// The result of a game of Brainfuck joust. A game consists of multiple rounds.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct GameResult {
    pub bot_a_points: i8,
    pub bot_b_points: i8,
}

impl GameResult {
    pub fn new() -> GameResult {
        GameResult {
            bot_a_points: 0,
            bot_b_points: 0,
        }
    }

    pub fn add_result_to_total(&mut self, round_result: &RoundResult) {
        if round_result.has_winner() {
            self.bot_a_points += if round_result.bot_a_lost { -1 } else { 1 };
            self.bot_b_points += if round_result.bot_b_lost { -1 } else { 1 };
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;
    use round::RoundResult;

    #[test]
    fn addResultToTotal_draw_bothStayAtZero() {
        let mut game_result = GameResult::new();
        game_result.add_result_to_total(&RoundResult::draw());
        assert_eq!(game_result,
                   GameResult {
                       bot_a_points: 0,
                       bot_b_points: 0,
                   });
    }

    /// GameResult should in practice not be passed a RoundResult for an ongoing round, but just in
    /// case, it should be robust enough not to break.
    #[test]
    fn addResultToTotal_roundOngoing_bothStayAtZero() {
        let mut game_result = GameResult::new();
        game_result.add_result_to_total(&RoundResult::round_ongoing());
        assert_eq!(game_result,
                   GameResult {
                       bot_a_points: 0,
                       bot_b_points: 0,
                   });
    }

    #[test]
    fn addResultToTotal_botAWins_zeroSumOnePoint() {
        let mut game_result = GameResult::new();
        game_result.add_result_to_total(&RoundResult::start_bot_wins());
        assert_eq!(game_result,
                   GameResult {
                       bot_a_points: 1,
                       bot_b_points: -1,
                   });
    }

    #[test]
    fn addResultToTotal_botBWins_zeroSumOnePoint() {
        let mut game_result = GameResult::new();
        game_result.add_result_to_total(&RoundResult::end_bot_wins());
        assert_eq!(game_result,
                   GameResult {
                       bot_a_points: -1,
                       bot_b_points: 1,
                   });
    }
}
