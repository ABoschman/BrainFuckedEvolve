//! Brainfuck Joust
//!
//! Rather than running a single round (which would have a random tape length from 10 to 30), tournaments generally run 42 rounds (or "jousts") between the two warriors, in order to make the results deterministic. 
//! Two things are varied: the tape has one of 21 different lengths (the integers from 10 to 30 inclusive); and one of the warriors may have its polarity exchanged, i.e., exchanging the meaning of + and -.
//!
//! The terminology used by this program is as follows: A complete game consists of 42 rounds. Each round consists of steps. At each step, both bots execute one command in their program. 
//! For performance reasons, it is possible to run an incomplete game, consisting of fewer than 42 rounds. An incomplete game gives non-deterministic results. 

#![allow(dead_code, unused_variables, unused_imports)]//TODO: Remove this debug line.

use bot::Bot;
use round::play_round;
use round::RoundResult;

/// Compares two bots in a (complete) game and returns the result. 
pub fn run_complete_game(bot_a: &Bot, bot_b: &Bot) -> GameResult {
    run_game(bot_a, bot_b, AllRounds::new() )
}

/// Compares two bots in a game consisting of the provided rounds. Returns the result of the game.
pub fn run_game<I>(bot_a: &Bot, bot_b: &Bot, rounds: I) -> GameResult
    where I: Iterator<Item=RoundParams> {
    rounds.fold(GameResult::new(), | mut game_result, round_params | {
        let round_result = play_round(bot_a, bot_b, &round_params);
        game_result.add_result_to_total(&round_result);
        game_result
    })
}

/// The result of a game of Brainfuck joust. A game consists of multiple rounds.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct GameResult {
    bot_a_points: i8,
    bot_b_points: i8,
}

impl GameResult {
    fn new() -> GameResult {
        GameResult { bot_a_points: 0, bot_b_points: 0 }
    }

    fn add_result_to_total(&mut self, round_result: &RoundResult) {
        if round_result.has_winner() {
            self.bot_a_points += if round_result.bot_a_lost { -1 } else { 1 };
            self.bot_b_points += if round_result.bot_b_lost { -1 } else { 1 };
        }
    }
}

/// Specifies the conditions of a single round of Brainfuck Jousting.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct RoundParams {
    tape_length: u8,
    invert_polarity: bool,
}

//===== Default rounds supplier, used for complete games:

const MIN_TAPE_LENGTH: u8 = 10;
const MAX_TAPE_LENGTH: u8 = 30;

/// An iterator that returns all rounds in a complete game, covering all possible tape lengths and both polarities.
pub struct AllRounds {
    tape_length: u8,
    invert_polarity: bool,
}

impl AllRounds {
    fn new() -> AllRounds {
        AllRounds {
            tape_length: MIN_TAPE_LENGTH,
            invert_polarity: false
        }
    }

    fn current_item(&self) -> RoundParams {
        RoundParams { tape_length: self.tape_length, invert_polarity: self.invert_polarity }
    }

    fn update_state(&mut self) {
        self.tape_length = if self.invert_polarity { self.tape_length + 1 } else { self.tape_length };
        self.invert_polarity = !self.invert_polarity;
    }

}

impl Iterator for AllRounds {
    type Item = RoundParams;

    fn next(&mut self) -> Option<RoundParams> {
        if self.tape_length <= MAX_TAPE_LENGTH {
            let params = self.current_item();
            self.update_state();
            Some(params)
        } else {
            None
        }
    }

}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;
    use round::RoundResult;

    #[test]
    fn allRounds_gives42Rounds() {
        assert_eq!(AllRounds::new().count(), 42);
    }

    #[test]
    fn allRounds_gives21ReversedPolarityRounds() {
        let all_inverted_rounds = AllRounds::new().filter(| round | { round.invert_polarity });
        assert_eq!(all_inverted_rounds.count(), 21);
    }

    #[test]
    fn allRounds_tapeIsAtLeast10Long() {
        assert!(AllRounds::new().all(| round | { round.tape_length >= 10 }));
    }

    #[test]
    fn allRounds_tapeIsAtMost30Long() {
        assert!(AllRounds::new().all(| round | { round.tape_length <= 30 }));
    }

    #[test]
    fn addResultToTotal_drawBothLose_bothStayAtZero() {
        let mut game_result = GameResult::new();
        game_result.add_result_to_total(&RoundResult::new(false, false));
        assert_eq!(game_result, GameResult {bot_a_points: 0, bot_b_points: 0});
    }

    #[test]
    fn addResultToTotal_drawNeitherLoses_bothStayAtZero() {
        let mut game_result = GameResult::new();
        game_result.add_result_to_total(&RoundResult::new(true, true));
        assert_eq!(game_result, GameResult {bot_a_points: 0, bot_b_points: 0});
    }

    #[test]
    fn addResultToTotal_botAWins_zeroSumOnePoint() {
        let mut game_result = GameResult::new();
        game_result.add_result_to_total(&RoundResult::new(false, true));
        assert_eq!(game_result, GameResult {bot_a_points: 1, bot_b_points: -1});
    }

    #[test]
    fn addResultToTotal_botBWins_zeroSumOnePoint() {
        let mut game_result = GameResult::new();
        game_result.add_result_to_total(&RoundResult::new(true, false));
        assert_eq!(game_result, GameResult {bot_a_points: -1, bot_b_points: 1});
    }
}