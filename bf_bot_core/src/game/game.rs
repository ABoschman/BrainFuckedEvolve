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
    run_game(bot_a, bot_b, &make_complete_rounds() )
}

/// Compares two bots in a game, using the given game parameters. Returns the result of the game.
pub fn run_game(bot_a: &Bot, bot_b: &Bot, rounds: &Iterator<Item=RoundParams>) -> GameResult {
    GameResult { bot_a_points: 0, bot_b_points: 0 }
}

/// The result of a full game of Brainfuck joust. A game consists of multiple rounds.
pub struct GameResult {
    bot_a_points: u8,
    bot_b_points: u8,
}

/// Specifies the parameters of a single round of Brainfuck Jousting.
pub struct RoundParams {
    tape_length: u8,
    invert_polarity: bool,
}

//===== Default rounds supplier, used for complete games:

fn make_complete_rounds() -> AllRounds {
    AllRounds {}
}

/// An iterator that returns all rounds in a complete game, covering all possible tape lengths and both polarities.
pub struct AllRounds {
    //TODO:...
}

impl Iterator for AllRounds {
    type Item = RoundParams;

    fn next(&mut self) -> Option<RoundParams> {
        //TODO:... spit out 42 rounds, tape lenghts [10-30], both polarities.
        None
    }
}