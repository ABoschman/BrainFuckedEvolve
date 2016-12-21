//! Brainfuck Joust
//!
//! Rather than running a single round (which would have a random tape length from 10 to 30), tournaments generally run 42 rounds (or "jousts") between the two warriors, in order to make the results deterministic. 
//! Two things are varied: the tape has one of 21 different lengths (the integers from 10 to 30 inclusive); and one of the warriors may have its polarity exchanged, i.e., exchanging the meaning of + and -.
//!
//! The terminology used by this program is as follows: A complete game consists of 42 rounds. Each round consists of steps. At each step, both bots execute one command in their program. 
//! For performance reasons, it is possible to run an incomplete game, consisting of fewer than 42 rounds. An incomplete game gives non-deterministic results. 

#![allow(dead_code, unused_variables, unused_imports)]//TODO: Remove this debug line.

use bot::Bot;

/// Compares two bots in a (complete) game and returns the result. 
pub fn run_complete_game(bot_a: &Bot, bot_b: &Bot) -> GameResult {
    run_game(bot_a, bot_b, &AllRounds::new() )
}

/// Compares two bots in a game consisting of the provided rounds. Returns the result of the game.
pub fn run_game(bot_a: &Bot, bot_b: &Bot, rounds: &Iterator<Item=RoundParams>) -> GameResult {
    //TODO:...
    GameResult { bot_a_points: 0, bot_b_points: 0 }
}

/// The result of a game of Brainfuck joust. A game consists of multiple rounds.
pub struct GameResult {
    bot_a_points: u8,
    bot_b_points: u8,
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

#[test]
#[allow(non_snake_case)]
fn allRounds_gives42Rounds() {
    assert_eq!(AllRounds::new().count(), 42);
}

#[test]
#[allow(non_snake_case)]
fn allRounds_gives21ReversedPolarityRounds() {
    let all_inverted_rounds = AllRounds::new().filter(| round | { round.invert_polarity });
    assert_eq!(all_inverted_rounds.count(), 21);
}

#[test]
#[allow(non_snake_case)]
fn allRounds_areAtLeastMinTapeLength() {
    assert!(AllRounds::new().all(| round | { round.tape_length >= MIN_TAPE_LENGTH }));
}

#[test]
#[allow(non_snake_case)]
fn allRounds_areAtMostMaxTapeLength() {
    assert!(AllRounds::new().all(| round | { round.tape_length <= MAX_TAPE_LENGTH }));
}